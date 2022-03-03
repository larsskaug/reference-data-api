use soup::prelude::*;
use std::fs;

fn main() -> Result<(), ureq::Error> {
    let url = "https://www.nrcs.usda.gov/wps/portal/nrcs/detail/?cid=nrcs143_013696";

    let html: String = ureq::get(url).call()?.into_string()?;

    let soup = Soup::new(&html);

    let div = soup
        .tag("div") // result must be a div
        .attr("id", "detail") // with id "foo"
        .find()
        .expect("Couldn't find tag");

    let data = div
        .tag("tr")
        .find_all()
        .skip(1)
        .map(|tr| {
            tr.tag("td")
                .find_all()
                .map(|td| td.text().replace('\n', "").replace('\t', ""))
                .collect::<Vec<_>>()
                .join(",")
        })
        .collect::<Vec<_>>().join("\n");

    //println!("{:?}", tr);
    fs::write("states.csv", data).expect("Unable to write file");
    Ok(())
}
