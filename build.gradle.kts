plugins {
  kotlin("jvm") version "2.2.20"
  scala
  application
}

group = "dev.miguely" // A company name, for example, `org.jetbrains`
version = "1.0" // Version to assign to the built artifact

repositories { // Sources of dependencies. See 2️⃣
  mavenCentral() // Maven Central Repository. See 3️⃣
}

dependencies { // All the libraries you want to use. See 4️⃣
  implementation("org.scala-lang:scala-library:2.13.15")
  // Copy dependencies' names after you find them in a repository
  testImplementation(kotlin("test")) // The Kotlin test library
}

tasks.test { // See 5️⃣
  useJUnitPlatform() // JUnitPlatform for tests. See 6️⃣
}

kotlin { // Extension for easy setup
  jvmToolchain(21) // Target version of generated JVM bytecode. See 7️⃣
}

application {
  mainClass.set("MainKt") // The main class of the application
}

tasks.register<JavaExec>("runDay") {
  group = "aoc"
  description = "Run any aoc<year>.DayXX.kt file with main() (e.g. -Pyear=2025 -Pday=01)"

  val year = providers.gradleProperty("year").orElse("2025")
  val day = providers.gradleProperty("day")

  mainClass.set(
    year.zip(day) { y, d ->
      val paddedDay = d.padStart(2, '0')
      "aoc${y}.Day${paddedDay}Kt"
    },
  )
  classpath = sourceSets["main"].runtimeClasspath
}
