use crate::prelude::*;

/// App home page
pub(crate) fn page_home(_contexts: Contexts) -> Html {
    set_title(&"Interior design 3d rendering and visualization specialist".to_string());
    html! {
        <>
            {title_secondary!("Interior design 3d rendering and visualization specialist")}
            <Paper class={CLASSES_PAGE_SECTION} elevation={ELEVATION_STANDARD}>
                <Paper>
                {paragraphs!(
                    "Thank you visiting my personal website.",
                    "Unfortunately, at this time there's not much to see here.",
                    html!{
                        <>
                            {"Be sure to visit my company site though, Gassler Design, for my "}
                            <Link title="Stoic Dreams" href="https://www.gassler.design">{"Interior design 3d rendering projects"}</Link>
                            {"."}
                        </>
                    }
                )}
                </Paper>
            </Paper>
        </>
    }
}
