pluginManagement {
    plugins {
        kotlin("jvm") version "2.2.0"
    }
}

plugins {
    // Apply the foojay-resolver plugin to allow automatic download of JDKs
    id("org.gradle.toolchains.foojay-resolver-convention") version "1.0.0"
}

rootProject.name = "senvend-api-kotlin"
// the "app" module contains the senvend-api library (published to Maven Central),
// the "example" module a CLI application demonstrating its usage
include("app", "example")
