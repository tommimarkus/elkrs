use std::env;
use std::fmt;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use elkrs_core::geometry::Point;
use elkrs_core::graph::{ElkGraph, ElkNode};
use elkrs_core::layout::LayoutError;
use elkrs_json::{from_str, to_string_pretty, JsonError};
use elkrs_layered::{LayeredLayout, LayoutAlgorithm};

#[path = "../../../crates/elkrs-layered/tests/support/fixtures.rs"]
mod fixtures;

const CANVAS_WIDTH: u32 = 1496;
const CANVAS_HEIGHT: u32 = 608;
const PAGE_MARGIN: i32 = 24;
const HEADER_HEIGHT: i32 = 40;
const PANEL_WIDTH: i32 = 700;
const PANEL_HEIGHT: i32 = 520;
const PANEL_GAP: i32 = 48;
const PANEL_PADDING: f64 = 36.0;

const BACKGROUND: Color = Color::rgb(248, 250, 252);
const PANEL_BACKGROUND: Color = Color::rgb(255, 255, 255);
const PANEL_BORDER: Color = Color::rgb(148, 163, 184);
const EDGE_COLOR: Color = Color::rgb(71, 85, 105);
const NODE_FILL: Color = Color::rgb(219, 234, 254);
const NODE_STROKE: Color = Color::rgb(37, 99, 235);
const PORT_FILL: Color = Color::rgb(15, 118, 110);
const TEXT_COLOR: Color = Color::rgb(15, 23, 42);

#[derive(Debug)]
pub enum VisualParityError {
    MissingJavaCommand,
    MissingFixtureSelection,
    ConflictingFixtureSelection,
    MissingValue(&'static str),
    UnknownArgument(String),
    UnknownFixture(String),
    JavaFailed(String),
    Io(std::io::Error),
    Json(JsonError),
    Layout(LayoutError),
}

impl fmt::Display for VisualParityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingJavaCommand => {
                write!(f, "set ELKRS_JAVA_ELK_COMMAND to a Java ELK JSON command")
            }
            Self::MissingFixtureSelection => {
                write!(f, "select a fixture with --fixture <name> or use --all")
            }
            Self::ConflictingFixtureSelection => write!(f, "use either --fixture or --all"),
            Self::MissingValue(flag) => write!(f, "{flag} requires a value"),
            Self::UnknownArgument(arg) => write!(f, "unknown argument: {arg}"),
            Self::UnknownFixture(name) => write!(f, "unknown fixture: {name}"),
            Self::JavaFailed(message) => write!(f, "Java ELK command failed: {message}"),
            Self::Io(error) => write!(f, "{error}"),
            Self::Json(error) => write!(f, "{error}"),
            Self::Layout(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for VisualParityError {}

impl From<std::io::Error> for VisualParityError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<JsonError> for VisualParityError {
    fn from(error: JsonError) -> Self {
        Self::Json(error)
    }
}

impl From<LayoutError> for VisualParityError {
    fn from(error: LayoutError) -> Self {
        Self::Layout(error)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct VisualFixture {
    pub id: &'static str,
    pub name: &'static str,
    build: fn() -> ElkGraph,
}

#[derive(Debug)]
struct CliOptions {
    fixture: Option<String>,
    all: bool,
    out_dir: PathBuf,
}

pub fn run<I, S>(args: I) -> Result<Vec<PathBuf>, VisualParityError>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let options = parse_args(args)?;
    let command =
        env::var("ELKRS_JAVA_ELK_COMMAND").map_err(|_| VisualParityError::MissingJavaCommand)?;
    let fixtures = selected_fixtures(&options)?;
    let mut paths = Vec::new();

    for fixture in fixtures {
        let input_graph = (fixture.build)();
        let input_json = to_string_pretty(&input_graph)?;
        let java_json = run_java_elk_command(&command, &input_json)?;
        let java_graph = from_str(&java_json)?;
        let mut rust_graph = input_graph;
        LayeredLayout.layout(&mut rust_graph)?;
        let path = write_comparison_png(&options.out_dir, fixture.name, &java_graph, &rust_graph)?;
        println!("{}", path.display());
        paths.push(path);
    }

    Ok(paths)
}

pub fn fixture_names() -> Vec<&'static str> {
    visual_fixtures()
        .into_iter()
        .map(|fixture| fixture.name)
        .collect()
}

pub fn select_fixture(name: &str) -> Result<VisualFixture, VisualParityError> {
    visual_fixtures()
        .into_iter()
        .find(|fixture| fixture.name == name)
        .ok_or_else(|| VisualParityError::UnknownFixture(name.to_owned()))
}

pub fn render_comparison_png(
    fixture_name: &str,
    java_graph: &ElkGraph,
    rust_graph: &ElkGraph,
) -> Result<Vec<u8>, VisualParityError> {
    let mut canvas = Canvas::new(CANVAS_WIDTH, CANVAS_HEIGHT, BACKGROUND);
    let java_bounds = Bounds::from_graph(java_graph);
    let rust_bounds = Bounds::from_graph(rust_graph);
    let bounds = java_bounds.union(rust_bounds).with_minimum_size();
    let panel_y = PAGE_MARGIN + HEADER_HEIGHT;
    let java_panel = Panel::new(PAGE_MARGIN, panel_y, PANEL_WIDTH, PANEL_HEIGHT, bounds);
    let rust_panel = Panel::new(
        PAGE_MARGIN + PANEL_WIDTH + PANEL_GAP,
        panel_y,
        PANEL_WIDTH,
        PANEL_HEIGHT,
        bounds,
    );

    draw_panel(
        &mut canvas,
        java_panel,
        "JAVA ELK",
        fixture_name,
        java_graph,
    );
    draw_panel(&mut canvas, rust_panel, "ELKRS", fixture_name, rust_graph);

    Ok(encode_png(canvas.width, canvas.height, &canvas.pixels))
}

pub fn write_comparison_png(
    output_dir: &Path,
    fixture_name: &str,
    java_graph: &ElkGraph,
    rust_graph: &ElkGraph,
) -> Result<PathBuf, VisualParityError> {
    fs::create_dir_all(output_dir)?;
    let path = output_dir.join(format!("{}.png", sanitize_file_name(fixture_name)));
    let png = render_comparison_png(fixture_name, java_graph, rust_graph)?;
    fs::write(&path, png)?;
    Ok(path)
}

fn parse_args<I, S>(args: I) -> Result<CliOptions, VisualParityError>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let mut fixture = None;
    let mut all = false;
    let mut out_dir = PathBuf::from(".cache/visual-parity");
    let mut args = args.into_iter().map(Into::into);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--fixture" => {
                if all {
                    return Err(VisualParityError::ConflictingFixtureSelection);
                }
                fixture = Some(
                    args.next()
                        .ok_or(VisualParityError::MissingValue("--fixture"))?,
                );
            }
            "--all" => {
                if fixture.is_some() {
                    return Err(VisualParityError::ConflictingFixtureSelection);
                }
                all = true;
            }
            "--out" => {
                out_dir = PathBuf::from(
                    args.next()
                        .ok_or(VisualParityError::MissingValue("--out"))?,
                );
            }
            "--help" | "-h" => return Err(VisualParityError::MissingFixtureSelection),
            other => return Err(VisualParityError::UnknownArgument(other.to_owned())),
        }
    }

    if !all && fixture.is_none() {
        return Err(VisualParityError::MissingFixtureSelection);
    }

    Ok(CliOptions {
        fixture,
        all,
        out_dir,
    })
}

fn selected_fixtures(options: &CliOptions) -> Result<Vec<VisualFixture>, VisualParityError> {
    if options.all {
        return Ok(visual_fixtures());
    }
    let fixture = options
        .fixture
        .as_deref()
        .ok_or(VisualParityError::MissingFixtureSelection)?;
    Ok(vec![select_fixture(fixture)?])
}

fn visual_fixtures() -> Vec<VisualFixture> {
    fixtures::parity_fixtures()
        .into_iter()
        .filter(|fixture| fixture.status == fixtures::ParityFixtureStatus::JavaComparable)
        .map(|fixture| VisualFixture {
            id: fixture.id,
            name: fixture.name,
            build: fixture.build,
        })
        .collect()
}

fn run_java_elk_command(command: &str, input: &str) -> Result<String, VisualParityError> {
    let mut child = Command::new(command)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    child
        .stdin
        .as_mut()
        .expect("stdin was piped")
        .write_all(input.as_bytes())?;

    let output = child.wait_with_output()?;
    if !output.status.success() {
        return Err(VisualParityError::JavaFailed(
            String::from_utf8_lossy(&output.stderr).trim().to_owned(),
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

fn draw_panel(
    canvas: &mut Canvas,
    panel: Panel,
    title: &str,
    fixture_name: &str,
    graph: &ElkGraph,
) {
    canvas.fill_rect(
        panel.x,
        panel.y,
        panel.width,
        panel.height,
        PANEL_BACKGROUND,
    );
    canvas.stroke_rect(panel.x, panel.y, panel.width, panel.height, PANEL_BORDER);
    canvas.draw_text(panel.x + 10, PAGE_MARGIN + 2, title, TEXT_COLOR, 3);
    canvas.draw_text(panel.x + 10, PAGE_MARGIN + 24, fixture_name, TEXT_COLOR, 2);
    draw_edges(canvas, panel, graph);
    for node in graph.nodes.values() {
        draw_node(canvas, panel, node);
    }
}

fn draw_edges(canvas: &mut Canvas, panel: Panel, graph: &ElkGraph) {
    for edge in graph.edges.values() {
        for section in &edge.sections {
            for points in section.points.windows(2) {
                let start = panel.map(points[0]);
                let end = panel.map(points[1]);
                canvas.draw_line(start.0, start.1, end.0, end.1, EDGE_COLOR, 2);
            }
        }
    }
}

fn draw_node(canvas: &mut Canvas, panel: Panel, node: &ElkNode) {
    let top_left = panel.map(node.position);
    let bottom_right = panel.map(Point::new(
        node.position.x + node.size.width,
        node.position.y + node.size.height,
    ));
    let x = top_left.0.min(bottom_right.0);
    let y = top_left.1.min(bottom_right.1);
    let width = (top_left.0 - bottom_right.0).abs().max(8);
    let height = (top_left.1 - bottom_right.1).abs().max(8);

    canvas.fill_rect(x, y, width, height, NODE_FILL);
    canvas.stroke_rect(x, y, width, height, NODE_STROKE);
    canvas.draw_text(x + 4, y + 5, node.id.as_str(), TEXT_COLOR, 1);

    for port in node.ports.values() {
        let port_top_left = panel.map(Point::new(
            node.position.x + port.position.x,
            node.position.y + port.position.y,
        ));
        let port_bottom_right = panel.map(Point::new(
            node.position.x + port.position.x + port.size.width.max(6.0),
            node.position.y + port.position.y + port.size.height.max(6.0),
        ));
        let port_x = port_top_left.0.min(port_bottom_right.0);
        let port_y = port_top_left.1.min(port_bottom_right.1);
        let port_width = (port_top_left.0 - port_bottom_right.0).abs().max(5);
        let port_height = (port_top_left.1 - port_bottom_right.1).abs().max(5);
        canvas.fill_rect(port_x, port_y, port_width, port_height, PORT_FILL);
    }

    for child in node.children.values() {
        draw_node(canvas, panel, child);
    }
}

fn sanitize_file_name(value: &str) -> String {
    let mut output = String::new();
    let mut previous_dash = false;
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            output.push(ch.to_ascii_lowercase());
            previous_dash = false;
        } else if !previous_dash {
            output.push('-');
            previous_dash = true;
        }
    }
    let trimmed = output.trim_matches('-');
    if trimmed.is_empty() {
        "comparison".to_owned()
    } else {
        trimmed.to_owned()
    }
}

#[derive(Debug, Clone, Copy)]
struct Panel {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    bounds: Bounds,
    scale: f64,
}

impl Panel {
    fn new(x: i32, y: i32, width: i32, height: i32, bounds: Bounds) -> Self {
        let available_width = (width as f64 - PANEL_PADDING * 2.0).max(1.0);
        let available_height = (height as f64 - PANEL_PADDING * 2.0).max(1.0);
        let scale = (available_width / bounds.width())
            .min(available_height / bounds.height())
            .max(0.001);
        Self {
            x,
            y,
            width,
            height,
            bounds,
            scale,
        }
    }

    fn map(self, point: Point) -> (i32, i32) {
        let x = self.x as f64 + PANEL_PADDING + (point.x - self.bounds.min_x) * self.scale;
        let y = self.y as f64 + PANEL_PADDING + (point.y - self.bounds.min_y) * self.scale;
        (x.round() as i32, y.round() as i32)
    }
}

#[derive(Debug, Clone, Copy)]
struct Bounds {
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,
}

impl Bounds {
    fn empty() -> Self {
        Self {
            min_x: f64::INFINITY,
            min_y: f64::INFINITY,
            max_x: f64::NEG_INFINITY,
            max_y: f64::NEG_INFINITY,
        }
    }

    fn from_graph(graph: &ElkGraph) -> Self {
        let mut bounds = Self::empty();
        for node in graph.nodes.values() {
            bounds.include_node(node);
        }
        for edge in graph.edges.values() {
            for section in &edge.sections {
                for point in &section.points {
                    bounds.include_point(*point);
                }
            }
        }
        if bounds.is_empty() {
            bounds.include_point(Point::new(0.0, 0.0));
            bounds.include_point(Point::new(100.0, 100.0));
        }
        bounds
    }

    fn include_node(&mut self, node: &ElkNode) {
        self.include_point(node.position);
        self.include_point(Point::new(
            node.position.x + node.size.width,
            node.position.y + node.size.height,
        ));
        for port in node.ports.values() {
            self.include_point(Point::new(
                node.position.x + port.position.x,
                node.position.y + port.position.y,
            ));
            self.include_point(Point::new(
                node.position.x + port.position.x + port.size.width.max(6.0),
                node.position.y + port.position.y + port.size.height.max(6.0),
            ));
        }
        for child in node.children.values() {
            self.include_node(child);
        }
    }

    fn include_point(&mut self, point: Point) {
        self.min_x = self.min_x.min(point.x);
        self.min_y = self.min_y.min(point.y);
        self.max_x = self.max_x.max(point.x);
        self.max_y = self.max_y.max(point.y);
    }

    fn union(self, other: Self) -> Self {
        Self {
            min_x: self.min_x.min(other.min_x),
            min_y: self.min_y.min(other.min_y),
            max_x: self.max_x.max(other.max_x),
            max_y: self.max_y.max(other.max_y),
        }
    }

    fn with_minimum_size(self) -> Self {
        let mut bounds = self;
        if bounds.width() < 1.0 {
            bounds.max_x = bounds.min_x + 1.0;
        }
        if bounds.height() < 1.0 {
            bounds.max_y = bounds.min_y + 1.0;
        }
        bounds
    }

    fn width(self) -> f64 {
        self.max_x - self.min_x
    }

    fn height(self) -> f64 {
        self.max_y - self.min_y
    }

    fn is_empty(self) -> bool {
        !self.min_x.is_finite()
    }
}

#[derive(Debug, Clone, Copy)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }
}

struct Canvas {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
}

impl Canvas {
    fn new(width: u32, height: u32, fill: Color) -> Self {
        let mut pixels = vec![0; width as usize * height as usize * 4];
        for chunk in pixels.chunks_exact_mut(4) {
            chunk[0] = fill.r;
            chunk[1] = fill.g;
            chunk[2] = fill.b;
            chunk[3] = fill.a;
        }
        Self {
            width,
            height,
            pixels,
        }
    }

    fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return;
        }
        let index = (y as usize * self.width as usize + x as usize) * 4;
        self.pixels[index] = color.r;
        self.pixels[index + 1] = color.g;
        self.pixels[index + 2] = color.b;
        self.pixels[index + 3] = color.a;
    }

    fn fill_rect(&mut self, x: i32, y: i32, width: i32, height: i32, color: Color) {
        for yy in y.max(0)..(y + height).min(self.height as i32) {
            for xx in x.max(0)..(x + width).min(self.width as i32) {
                self.set_pixel(xx, yy, color);
            }
        }
    }

    fn stroke_rect(&mut self, x: i32, y: i32, width: i32, height: i32, color: Color) {
        self.draw_line(x, y, x + width, y, color, 1);
        self.draw_line(x + width, y, x + width, y + height, color, 1);
        self.draw_line(x + width, y + height, x, y + height, color, 1);
        self.draw_line(x, y + height, x, y, color, 1);
    }

    fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: Color, radius: i32) {
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let steps = dx.max(dy).max(1);
        for step in 0..=steps {
            let t = step as f64 / steps as f64;
            let x = x0 as f64 + (x1 - x0) as f64 * t;
            let y = y0 as f64 + (y1 - y0) as f64 * t;
            self.fill_rect(
                x.round() as i32 - radius,
                y.round() as i32 - radius,
                radius * 2 + 1,
                radius * 2 + 1,
                color,
            );
        }
    }

    fn draw_text(&mut self, x: i32, y: i32, text: &str, color: Color, scale: i32) {
        let mut cursor_x = x;
        for ch in text.chars() {
            if let Some(glyph) = glyph(ch) {
                for (row_index, row) in glyph.iter().enumerate() {
                    for column in 0..5 {
                        if row & (1 << (4 - column)) != 0 {
                            self.fill_rect(
                                cursor_x + column * scale,
                                y + row_index as i32 * scale,
                                scale,
                                scale,
                                color,
                            );
                        }
                    }
                }
            }
            cursor_x += 6 * scale;
        }
    }
}

fn glyph(ch: char) -> Option<[u8; 7]> {
    let ch = ch.to_ascii_uppercase();
    let glyph = match ch {
        'A' => [
            0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
        ],
        'B' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110,
        ],
        'C' => [
            0b01110, 0b10001, 0b10000, 0b10000, 0b10000, 0b10001, 0b01110,
        ],
        'D' => [
            0b11110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11110,
        ],
        'E' => [
            0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111,
        ],
        'F' => [
            0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000,
        ],
        'G' => [
            0b01110, 0b10001, 0b10000, 0b10111, 0b10001, 0b10001, 0b01110,
        ],
        'H' => [
            0b10001, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
        ],
        'I' => [
            0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b11111,
        ],
        'J' => [
            0b00111, 0b00010, 0b00010, 0b00010, 0b10010, 0b10010, 0b01100,
        ],
        'K' => [
            0b10001, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010, 0b10001,
        ],
        'L' => [
            0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111,
        ],
        'M' => [
            0b10001, 0b11011, 0b10101, 0b10101, 0b10001, 0b10001, 0b10001,
        ],
        'N' => [
            0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001, 0b10001,
        ],
        'O' => [
            0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110,
        ],
        'P' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000, 0b10000,
        ],
        'Q' => [
            0b01110, 0b10001, 0b10001, 0b10001, 0b10101, 0b10010, 0b01101,
        ],
        'R' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001,
        ],
        'S' => [
            0b01111, 0b10000, 0b10000, 0b01110, 0b00001, 0b00001, 0b11110,
        ],
        'T' => [
            0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100,
        ],
        'U' => [
            0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110,
        ],
        'V' => [
            0b10001, 0b10001, 0b10001, 0b10001, 0b01010, 0b01010, 0b00100,
        ],
        'W' => [
            0b10001, 0b10001, 0b10001, 0b10101, 0b10101, 0b10101, 0b01010,
        ],
        'X' => [
            0b10001, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001, 0b10001,
        ],
        'Y' => [
            0b10001, 0b10001, 0b01010, 0b00100, 0b00100, 0b00100, 0b00100,
        ],
        'Z' => [
            0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b10000, 0b11111,
        ],
        '0' => [
            0b01110, 0b10001, 0b10011, 0b10101, 0b11001, 0b10001, 0b01110,
        ],
        '1' => [
            0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110,
        ],
        '2' => [
            0b01110, 0b10001, 0b00001, 0b00010, 0b00100, 0b01000, 0b11111,
        ],
        '3' => [
            0b11110, 0b00001, 0b00001, 0b01110, 0b00001, 0b00001, 0b11110,
        ],
        '4' => [
            0b00010, 0b00110, 0b01010, 0b10010, 0b11111, 0b00010, 0b00010,
        ],
        '5' => [
            0b11111, 0b10000, 0b10000, 0b11110, 0b00001, 0b00001, 0b11110,
        ],
        '6' => [
            0b01110, 0b10000, 0b10000, 0b11110, 0b10001, 0b10001, 0b01110,
        ],
        '7' => [
            0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b01000, 0b01000,
        ],
        '8' => [
            0b01110, 0b10001, 0b10001, 0b01110, 0b10001, 0b10001, 0b01110,
        ],
        '9' => [
            0b01110, 0b10001, 0b10001, 0b01111, 0b00001, 0b00001, 0b01110,
        ],
        '-' => [
            0b00000, 0b00000, 0b00000, 0b11111, 0b00000, 0b00000, 0b00000,
        ],
        '_' => [
            0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b11111,
        ],
        ':' => [
            0b00000, 0b00100, 0b00100, 0b00000, 0b00100, 0b00100, 0b00000,
        ],
        '.' => [
            0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00100, 0b00100,
        ],
        ' ' => [0; 7],
        _ => return None,
    };
    Some(glyph)
}

fn encode_png(width: u32, height: u32, rgba: &[u8]) -> Vec<u8> {
    let mut png = Vec::new();
    png.extend_from_slice(b"\x89PNG\r\n\x1a\n");

    let mut ihdr = Vec::with_capacity(13);
    ihdr.extend_from_slice(&width.to_be_bytes());
    ihdr.extend_from_slice(&height.to_be_bytes());
    ihdr.extend_from_slice(&[8, 6, 0, 0, 0]);
    write_chunk(&mut png, b"IHDR", &ihdr);

    let row_len = width as usize * 4;
    let mut filtered = Vec::with_capacity((row_len + 1) * height as usize);
    for row in rgba.chunks_exact(row_len) {
        filtered.push(0);
        filtered.extend_from_slice(row);
    }
    let compressed = zlib_store(&filtered);
    write_chunk(&mut png, b"IDAT", &compressed);
    write_chunk(&mut png, b"IEND", &[]);
    png
}

fn write_chunk(png: &mut Vec<u8>, kind: &[u8; 4], data: &[u8]) {
    png.extend_from_slice(&(data.len() as u32).to_be_bytes());
    png.extend_from_slice(kind);
    png.extend_from_slice(data);
    let mut crc_input = Vec::with_capacity(kind.len() + data.len());
    crc_input.extend_from_slice(kind);
    crc_input.extend_from_slice(data);
    png.extend_from_slice(&crc32(&crc_input).to_be_bytes());
}

fn zlib_store(data: &[u8]) -> Vec<u8> {
    let mut output = Vec::new();
    output.extend_from_slice(&[0x78, 0x01]);
    for (index, chunk) in data.chunks(65_535).enumerate() {
        let final_block = index == data.len().saturating_sub(1) / 65_535;
        output.push(if final_block { 0x01 } else { 0x00 });
        let len = chunk.len() as u16;
        output.extend_from_slice(&len.to_le_bytes());
        output.extend_from_slice(&(!len).to_le_bytes());
        output.extend_from_slice(chunk);
    }
    output.extend_from_slice(&adler32(data).to_be_bytes());
    output
}

fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xffff_ffff;
    for byte in data {
        crc ^= *byte as u32;
        for _ in 0..8 {
            if crc & 1 == 1 {
                crc = (crc >> 1) ^ 0xedb8_8320;
            } else {
                crc >>= 1;
            }
        }
    }
    !crc
}

fn adler32(data: &[u8]) -> u32 {
    const MOD: u32 = 65_521;
    let mut a = 1u32;
    let mut b = 0u32;
    for byte in data {
        a = (a + *byte as u32) % MOD;
        b = (b + a) % MOD;
    }
    (b << 16) | a
}

#[cfg(test)]
mod tests {
    use std::fs;

    use elkrs_core::geometry::{Point, Size};
    use elkrs_core::graph::{ElementId, ElementRef, ElkEdge, ElkEdgeSection, ElkGraph, ElkNode};

    use super::*;

    #[test]
    fn fixture_names_include_java_comparable_chain() {
        let names = fixture_names();

        assert!(names.contains(&"chain"));
    }

    #[test]
    fn select_fixture_rejects_unknown_name() {
        let error = select_fixture("missing-fixture").unwrap_err();

        assert!(error.to_string().contains("unknown fixture"));
    }

    #[test]
    fn render_comparison_png_has_png_signature_and_dimensions() {
        let left = positioned_graph("left");
        let right = positioned_graph("right");

        let png = render_comparison_png("fixture", &left, &right).unwrap();

        assert_eq!(&png[..8], b"\x89PNG\r\n\x1a\n");
        assert!(png.len() > 100);
        assert_eq!(u32::from_be_bytes(png[16..20].try_into().unwrap()), 1496);
        assert_eq!(u32::from_be_bytes(png[20..24].try_into().unwrap()), 608);
    }

    #[test]
    fn write_comparison_png_creates_non_empty_file() {
        let output_dir =
            std::env::temp_dir().join(format!("elkrs-visual-parity-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&output_dir);
        let left = positioned_graph("left");
        let right = positioned_graph("right");

        let path = write_comparison_png(&output_dir, "sample fixture", &left, &right).unwrap();

        let bytes = fs::read(&path).unwrap();
        assert_eq!(&bytes[..8], b"\x89PNG\r\n\x1a\n");
        assert!(bytes.len() > 100);

        let _ = fs::remove_dir_all(output_dir);
    }

    fn positioned_graph(id: &str) -> ElkGraph {
        let mut graph = ElkGraph::new(id);
        let mut a = ElkNode::new("a");
        a.position = Point::new(0.0, 0.0);
        a.size = Size::new(50.0, 30.0);
        let mut b = ElkNode::new("b");
        b.position = Point::new(140.0, 0.0);
        b.size = Size::new(50.0, 30.0);
        graph.add_node(a);
        graph.add_node(b);
        let mut edge = ElkEdge::new(
            "ab",
            ElementRef::Node(ElementId::from("a")),
            ElementRef::Node(ElementId::from("b")),
        );
        edge.sections.push(ElkEdgeSection {
            points: vec![Point::new(50.0, 15.0), Point::new(140.0, 15.0)],
        });
        graph.add_edge(edge);
        graph
    }
}
