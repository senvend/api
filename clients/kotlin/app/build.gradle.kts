import com.vanniktech.maven.publish.JavadocJar
import com.vanniktech.maven.publish.KotlinJvm

plugins {
    kotlin("jvm")
    id("com.vanniktech.maven.publish") version "0.37.0"
}

repositories {
    // Use Maven Central for resolving dependencies.
    mavenCentral()
}

sourceSets.main {
    java.srcDirs("$projectDir/senvend_api/java", "$projectDir/senvend_api/kotlin", "$projectDir/senvend_api/grpc-kotlin", "$projectDir/senvend_api/grpc-java")
}

// grpc protoc plugins used as buf "local" plugins, see ../buf.gen.yaml
val grpcPlugins = configurations.create("grpcPlugins") {
    isTransitive = false
}

val protocArch =
    when (val arch = System.getProperty("os.arch")) {
        "amd64", "x86_64" -> "x86_64"
        "aarch64" -> "aarch_64"
        else -> error("unsupported architecture: " + arch)
    }

dependencies {
    grpcPlugins("io.grpc:protoc-gen-grpc-java:1.84.0:linux-x86_64@exe")
    grpcPlugins("io.grpc:protoc-gen-grpc-java:1.84.0:linux-aarch_64@exe")
    grpcPlugins("io.grpc:protoc-gen-grpc-kotlin:1.5.0:jdk8@jar")

    api(kotlin("stdlib-jdk8"))

    api("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.11.0")

    api("com.google.protobuf:protobuf-kotlin:4.36.1")

    api("io.netty:netty-handler:4.2.18.Final")

    api("io.grpc:grpc-netty:1.84.0")

    api("io.grpc:grpc-kotlin-stub:1.5.0")
    api("io.grpc:grpc-core:1.84.0")
    api("io.grpc:grpc-protobuf:1.84.0")
    api("io.grpc:grpc-stub:1.84.0")
}

// downloads the grpc protoc plugins into ../.plugins, where buf.gen.yaml expects them
tasks.register<Sync>("downloadGrpcPlugins") {
    from(grpcPlugins) {
        include("*-linux-$protocArch.exe", "*.jar")
    }
    into(rootProject.layout.projectDirectory.dir(".plugins"))
    rename("protoc-gen-grpc-java-.*\\.exe", "protoc-gen-grpc-java")
    rename("protoc-gen-grpc-kotlin-.*\\.jar", "protoc-gen-grpc-kotlin.jar")
    filePermissions { unix("rwxr-xr-x") }
}

// Apply a specific Java toolchain to ease working on different environments.
java {
    toolchain {
        languageVersion = JavaLanguageVersion.of(21)
    }
}

mavenPublishing {
    publishToMavenCentral()
    // only sign when a key is provided (real releases), so that local
    // verification via publishToMavenLocal works without one
    if (providers.gradleProperty("signingInMemoryKey").isPresent) {
        signAllPublications()
    }
    coordinates("com.senbax.senvend", "senvend-api", project.version.toString())
    configure(KotlinJvm(javadocJar = JavadocJar.Javadoc(), sourcesJar = true))
    pom {
        name.set("SENVEND API")
        description.set("Public APIs to interact with the SENVEND ecosystem")
        url.set("https://github.com/senvend/api")
        licenses {
            license {
                name.set("MIT License")
                url.set("https://opensource.org/license/mit")
            }
            license {
                name.set("Apache License, Version 2.0")
                url.set("https://www.apache.org/licenses/LICENSE-2.0")
            }
        }
        developers {
            developer {
                id.set("senvend")
                name.set("SENVEND")
                email.set("api@senvend.com")
            }
        }
        scm {
            url.set("https://github.com/senvend/api")
            connection.set("scm:git:https://github.com/senvend/api.git")
            developerConnection.set("scm:git:git@github.com:senvend/api.git")
        }
    }
}
