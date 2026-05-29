package org.elkrs.tools.javaelkjson;

import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collection;
import java.util.Comparator;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;

import com.google.gson.Gson;
import com.google.gson.GsonBuilder;
import org.eclipse.elk.alg.layered.options.LayeredMetaDataProvider;
import org.eclipse.elk.core.RecursiveGraphLayoutEngine;
import org.eclipse.elk.core.data.LayoutAlgorithmData;
import org.eclipse.elk.core.data.LayoutMetaDataService;
import org.eclipse.elk.core.data.LayoutOptionData;
import org.eclipse.elk.core.options.CoreOptions;
import org.eclipse.elk.core.util.BasicProgressMonitor;
import org.eclipse.elk.graph.ElkNode;
import org.eclipse.elk.graph.json.ElkGraphJson;

public final class JavaElkJsonRunner {
    private static final String ELK_VERSION = "0.11.0";
    private static final String LAYERED_ALGORITHM_ID = "org.eclipse.elk.layered";
    private static final Gson GSON = new GsonBuilder().serializeNulls().setPrettyPrinting().create();

    private JavaElkJsonRunner() {
    }

    public static void main(String[] args) {
        try {
            registerLayeredMetadata();

            if (args.length == 1 && "--metadata".equals(args[0])) {
                System.out.println(GSON.toJson(exportMetadata()));
                return;
            }

            if (args.length != 0) {
                throw new IllegalArgumentException("java-elk-json reads JSON from stdin or exactly one --metadata argument");
            }

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

    private static Map<String, Object> exportMetadata() {
        LayoutMetaDataService service = LayoutMetaDataService.getInstance();
        LayoutAlgorithmData algorithm = service.getAlgorithmData(LAYERED_ALGORITHM_ID);
        if (algorithm == null) {
            throw new IllegalStateException("missing ELK Layered metadata for " + LAYERED_ALGORITHM_ID);
        }

        Map<String, Object> metadata = new LinkedHashMap<>();
        metadata.put("elkVersion", ELK_VERSION);
        metadata.put("algorithm", algorithmMetadata(algorithm));
        metadata.put("knownOptions", knownOptionsMetadata(service, algorithm));
        metadata.put("generatedBy", "tools/java-elk-json-runner --metadata");
        return metadata;
    }

    private static Map<String, Object> algorithmMetadata(LayoutAlgorithmData algorithm) {
        Map<String, Object> metadata = new LinkedHashMap<>();
        metadata.put("id", algorithm.getId());
        metadata.put("name", algorithm.getName());
        metadata.put("description", algorithm.getDescription());
        metadata.put("categoryId", algorithm.getCategoryId());
        metadata.put("bundleName", algorithm.getBundleName());
        metadata.put("definingBundleId", algorithm.getDefiningBundleId());
        metadata.put("supportedFeatures", sortedEnumNames(algorithm.getSupportedFeatures()));
        return metadata;
    }

    private static List<Map<String, Object>> knownOptionsMetadata(
            LayoutMetaDataService service, LayoutAlgorithmData algorithm) {
        List<String> optionIds = new ArrayList<>(algorithm.getKnownOptionIds());
        optionIds.sort(Comparator.naturalOrder());

        List<Map<String, Object>> options = new ArrayList<>();
        for (String optionId : optionIds) {
            LayoutOptionData option = service.getOptionData(optionId);
            if (option == null) {
                Map<String, Object> missing = new LinkedHashMap<>();
                missing.put("id", optionId);
                missing.put("missingMetadata", missingMetadata(optionId));
                options.add(missing);
            } else {
                options.add(optionMetadata(algorithm, option));
            }
        }
        return options;
    }

    private static Map<String, Object> optionMetadata(LayoutAlgorithmData algorithm, LayoutOptionData option) {
        Map<String, Object> metadata = new LinkedHashMap<>();
        metadata.put("id", option.getId());
        metadata.put("name", option.getName());
        metadata.put("description", option.getDescription());
        metadata.put("group", option.getGroup());
        metadata.put("type", enumName(option.getType()));
        metadata.put("targets", sortedEnumNames(option.getTargets()));
        metadata.put("default", jsonValue(option.getDefault()));
        metadata.put("algorithmDefault", jsonValue(algorithm.getDefaultValue(option.getId())));
        metadata.put("lowerBound", jsonValue(option.getLowerBound()));
        metadata.put("upperBound", jsonValue(option.getUpperBound()));
        metadata.put("optionClass", className(option.getOptionClass()));
        metadata.put("visibility", enumName(option.getVisibility()));
        metadata.put("legacyIds", legacyIds(option));
        metadata.put("enumValues", enumValues(option));
        return metadata;
    }

    private static Map<String, Object> missingMetadata(String optionId) {
        Map<String, Object> metadata = new LinkedHashMap<>();
        metadata.put("id", optionId);
        metadata.put("reason", "LayoutMetaDataService returned null for known option id");
        return metadata;
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

    private static List<String> enumValues(LayoutOptionData option) {
        List<String> values = new ArrayList<>();
        for (int index = 0; index < option.getEnumValueCount(); index++) {
            values.add(enumName(option.getEnumValue(index)));
        }
        values.sort(Comparator.naturalOrder());
        return values;
    }

    private static List<String> legacyIds(LayoutOptionData option) {
        String[] legacyIds = option.getLegacyIds();
        if (legacyIds == null) {
            return List.of();
        }
        return sortedStrings(Arrays.asList(legacyIds));
    }

    private static List<String> sortedEnumNames(Collection<? extends Enum<?>> values) {
        List<String> names = new ArrayList<>();
        for (Enum<?> value : values) {
            names.add(enumName(value));
        }
        names.sort(Comparator.naturalOrder());
        return names;
    }

    private static List<String> sortedStrings(Collection<String> values) {
        List<String> sorted = new ArrayList<>(values);
        sorted.sort(Comparator.naturalOrder());
        return sorted;
    }

    private static String enumName(Enum<?> value) {
        return value == null ? null : value.name();
    }

    private static String className(Class<?> value) {
        return value == null ? null : value.getName();
    }

    private static Object jsonValue(Object value) {
        if (value == null || value instanceof Boolean || value instanceof Number || value instanceof String) {
            return value;
        }
        if (value instanceof Enum<?>) {
            return enumName((Enum<?>) value);
        }
        if (value instanceof Collection<?>) {
            List<Object> values = new ArrayList<>();
            for (Object item : (Collection<?>) value) {
                values.add(jsonValue(item));
            }
            return values;
        }
        return value.toString();
    }
}
