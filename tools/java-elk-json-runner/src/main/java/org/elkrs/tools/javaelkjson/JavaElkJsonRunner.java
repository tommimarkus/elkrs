package org.elkrs.tools.javaelkjson;

import java.io.IOException;
import java.nio.charset.StandardCharsets;

import org.eclipse.elk.alg.layered.options.LayeredMetaDataProvider;
import org.eclipse.elk.core.RecursiveGraphLayoutEngine;
import org.eclipse.elk.core.data.LayoutMetaDataService;
import org.eclipse.elk.core.options.CoreOptions;
import org.eclipse.elk.core.util.BasicProgressMonitor;
import org.eclipse.elk.graph.ElkNode;
import org.eclipse.elk.graph.json.ElkGraphJson;

public final class JavaElkJsonRunner {
    private static final String LAYERED_ALGORITHM_ID = "org.eclipse.elk.layered";

    private JavaElkJsonRunner() {
    }

    public static void main(String[] args) {
        try {
            if (args.length != 0) {
                throw new IllegalArgumentException("java-elk-json reads JSON from stdin and accepts no arguments");
            }

            registerLayeredMetadata();

            String input = readStdin();
            if (input.isBlank()) {
                throw new IllegalArgumentException("stdin must contain an ELK JSON graph");
            }

            ElkNode graph = ElkGraphJson.forGraph(input).toElk();
            ensureLayeredAlgorithm(graph);

            new RecursiveGraphLayoutEngine().layout(graph, new BasicProgressMonitor());

            String output = ElkGraphJson.forGraph(graph)
                    .omitLayout(false)
                    .omitZeroDimension(false)
                    .omitZeroPositions(false)
                    .shortLayoutOptionKeys(false)
                    .prettyPrint(true)
                    .toJson();

            System.out.print(output);
        } catch (Exception error) {
            System.err.println("java-elk-json failed: " + error.getMessage());
            error.printStackTrace(System.err);
            System.exit(1);
        }
    }

    private static void registerLayeredMetadata() {
        LayoutMetaDataService service = LayoutMetaDataService.getInstance();
        service.registerLayoutMetaDataProviders(new CoreOptions());
        service.registerLayoutMetaDataProviders(new LayeredMetaDataProvider());
    }

    private static void ensureLayeredAlgorithm(ElkNode graph) {
        String algorithm = graph.getProperty(CoreOptions.ALGORITHM);
        if (algorithm == null || algorithm.isBlank()) {
            graph.setProperty(CoreOptions.ALGORITHM, LAYERED_ALGORITHM_ID);
        }
    }

    private static String readStdin() throws IOException {
        return new String(System.in.readAllBytes(), StandardCharsets.UTF_8);
    }
}
