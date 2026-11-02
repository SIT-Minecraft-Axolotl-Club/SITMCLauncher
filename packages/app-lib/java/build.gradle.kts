import java.security.MessageDigest

plugins {
    java
    id("com.diffplug.spotless") version "8.0.0"
    id("com.gradleup.shadow") version "9.2.2"
}

repositories {
    mavenCentral()
}

dependencies {
    implementation("org.ow2.asm:asm:9.9")
    implementation("org.ow2.asm:asm-tree:9.9")
    implementation("com.google.code.gson:gson:2.13.2")

    testImplementation(libs.junit.jupiter)
    testRuntimeOnly("org.junit.platform:junit-platform-launcher")
}

java {
    toolchain {
        languageVersion = JavaLanguageVersion.of(17)
    }
}

tasks.withType<JavaCompile>().configureEach {
    options.release = 8
    options.compilerArgs.addAll(listOf("-Xlint:all", "-Werror"))
}

spotless {
    java {
        palantirJavaFormat()
        removeUnusedImports()
    }
}

tasks.jar {
    enabled = false
}

tasks.shadowJar {
    archiveFileName = "theseus.jar"
    manifest {
        attributes["Premain-Class"] = "com.modrinth.theseus.agent.TheseusAgent"
    }

    addMultiReleaseAttribute = false
    enableAutoRelocation = true
    relocationPrefix = "com.modrinth.theseus.shadow"
}

val authlibInjector by tasks.registering {
    notCompatibleWithConfigurationCache("Downloads and verifies a pinned external Java agent")
    val output = layout.buildDirectory.file("libs/authlib-injector.jar")
    // The society's authlib-injector fork: same agent entry point as upstream,
    // built for the society's Yggdrasil endpoints.
    inputs.property("version", "union-1.4.0")
    inputs.property(
        "sha256",
        "a81609e39de1f57a4c522e6de5b02bf4037fcb303481730498d730db5e747687",
    )
    outputs.file(output)

    doLast {
        val bytes = uri(
            "https://www.sitmc.club/sit/authlib-injector-union-1.4.0.jar",
        ).toURL().readBytes()
        val checksum = MessageDigest.getInstance("SHA-256")
            .digest(bytes)
            .joinToString("") { "%02x".format(it) }
        check(checksum == inputs.properties["sha256"]) {
            "authlib-injector checksum mismatch: $checksum"
        }
        output.get().asFile.apply {
            parentFile.mkdirs()
            writeBytes(bytes)
        }
    }
}

tasks.build {
    dependsOn(authlibInjector)
}

tasks.named<Test>("test") {
    useJUnitPlatform()
}

tasks.withType<AbstractArchiveTask>().configureEach {
    isPreserveFileTimestamps = false
    isReproducibleFileOrder = true
}
