use std::{
    fs::{create_dir_all, write},
    io::Error,
    path::Path,
};

use conspire::constitutive::solid::{
    elastic::doc::{DOC as ELASTIC_DOC, almansi_hamel, hencky as hencky_elastic},
    hyperelastic::{
        doc::DOC as HYPERELASTIC_DOC, doc::arruda_boyce, doc::fung, doc::gent, doc::hencky,
        doc::mooney_rivlin, doc::neo_hookean, doc::saint_venant_kirchhoff,
    },
};

fn main() -> Result<(), Error> {
    let category_docs = [
        ("constitutive/solid/elastic", ELASTIC_DOC),
        ("constitutive/solid/hyperelastic", HYPERELASTIC_DOC),
    ];
    category_docs.iter().try_for_each(|(path, doc)| {
        create_dir_all(Path::new(format!("src/{path}").as_str()))?;
        write(Path::new(format!("src/{path}/doc.md").as_str()), doc)
    })?;
    let models = [
        almansi_hamel(),
        arruda_boyce(),
        fung(),
        gent(),
        hencky(),
        hencky_elastic(),
        mooney_rivlin(),
        neo_hookean(),
        saint_venant_kirchhoff(),
    ];
    models.iter().try_for_each(|model| {
        let path = model[0][0];
        create_dir_all(Path::new(format!("src/{path}").as_str()))?;
        write(
            Path::new(format!("src/{path}/doc.md").as_str()),
            model[0][1].replace("$`", "$").replace("`$", "$"),
        )?;
        model.iter().skip(1).try_for_each(|[method, doc]| {
            if doc.is_empty() {
                write(
                    Path::new(format!("src/{path}/{method}.md").as_str()),
                    "@private",
                )
            } else {
                write(
                    Path::new(format!("src/{path}/{method}.md").as_str()),
                    doc.replace("```math", "$$")
                        .replace("```", "$$")
                        .replace("\\begin{aligned}", "")
                        .replace("\\end{aligned}", "")
                        .replace("&", "")
                        .replace("\\\\", "")
                        .replace("\n", ""),
                )
            }
        })
    })
}
