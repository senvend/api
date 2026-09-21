pluginManagement {
    plugins {
        kotlin("jvm") version "2.2.0"
    }
}

plugins {
    // Apply the foojay-resolver plugin to allow automatic download of JDKs
    id("org.gradle.toolchains.foojay-resolver-convention") version "1.0.0"
}

rootProject.name = "example"
include("app")
