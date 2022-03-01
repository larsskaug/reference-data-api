package scrape

object States {
    def main(args: Array[String]): Unit = {

import org.jsoup._
import java.nio.file.{Files, Paths}
import scala.collection.JavaConverters


val url = "https://www.nrcs.usda.gov/wps/portal/nrcs/detail/?cid=nrcs143_013696"

val doc = Jsoup.connect(url).get

val statesTbl = doc.select("#detail > table").select("td")


val stateRef =statesTbl.toArray.map(_.asInstanceOf[org.jsoup.nodes.Element])
    .map(_.text).grouped(3).map(row => (row(0), row(1), row(2).toInt).productIterator.mkString(",")).toSeq

Files.writeString(Paths.get("states.csv"), stateRef.mkString("\n") )


    }
}
