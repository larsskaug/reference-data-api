lazy val scrapeStateReferenceData = (project in file("."))
    .enablePlugins(NativeImagePlugin)
    .settings(
        name := "scrape-state-fips",
        version := "0.1",
        scalaVersion := "2.12.15",
        Compile / mainClass := Some("scrape.States"),
        nativeImageOptions ++= Seq("--initialize-at-build-time",
                "-H:EnableURLProtocols=http",
                "-H:EnableURLProtocols=https",
                "--enable-url-protocols=http,https",
                "--enable-https",
                "--enable-http")
    )

libraryDependencies += "org.jsoup" % "jsoup" % "1.14.3"
