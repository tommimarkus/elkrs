plugins {
    application
}

group = "org.elkrs.tools"
version = "0.1.0"

java {
    sourceCompatibility = JavaVersion.VERSION_17
    targetCompatibility = JavaVersion.VERSION_17
}

dependencies {
    implementation("org.eclipse.elk:org.eclipse.elk.core:0.11.0")
    implementation("org.eclipse.elk:org.eclipse.elk.graph:0.11.0")
    implementation("org.eclipse.elk:org.eclipse.elk.graph.json:0.11.0")
    implementation("org.eclipse.elk:org.eclipse.elk.alg.layered:0.11.0")
}

dependencyLocking {
    lockAllConfigurations()
}

application {
    mainClass.set("org.elkrs.tools.javaelkjson.JavaElkJsonRunner")
    applicationName = "java-elk-json-runner"
}
