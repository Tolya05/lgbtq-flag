use yew::prelude::*;
use rand::Rng;

fn get_flag_link() -> (String, String) {
    let ids = [
        "transgender_1999",
        "pride_1979",
        "pride_1978_sevenStripes",
        "pride_1978_eightStripes",
        "polysexual_2012",
        "pansexual_2010",
        "nonbinary_2014",
        "lesbian_2019",
        "lesbian_2018",
        "lesbian_2010",
        "intersex_2013",
        "genderqueer_2011",
        "genderfluid_2012",
        "bisexual_1998",
        "asexual_2010",
        "aromantic_2014",
        "agender_2014",
    ];

    let names = [
        "Transgender",
        "Pride",
        "Pride",
        "Pride",
        "Polysexual",
        "Pansexual",
        "Non-Binary",
        "Lesbian",
        "Lesbian",
        "Lesbian",
        "Intersex",
        "Genderqueer",
        "Genderfluid",
        "Bisexual",
        "Asexual",
        "Aromantic",
        "Agender",
    ];

    let mut rng = rand::thread_rng();
    let flag_number = rng.gen_range(0..ids.len());
    let flag_id = ids[flag_number];
    let name = names[flag_number].to_string();

    let flag_link = format!("https://pride.dev/api/flags/{flag_id}/SVG");
    (flag_link, name)
}

#[function_component(App)]
pub fn app() -> Html {
    
    let flag_link_list = get_flag_link();
    let flag_link = &flag_link_list.0.clone();
    let name = &flag_link_list.1.clone();

    let onclick = Callback::from(move |_| {
        let window = web_sys::window().unwrap();
        window.location().reload().unwrap();
    });

    html! {
        <main>
        <div class="w3-center w3-black">
            <p></p>
            <div>
                <h1>{ "Welcome to Pride Flag Generator" }</h1>
                <h2>{ format!("This flag is {}", name) }</h2>
            </div>
            <div>
                <img src={flag_link.clone()} style="width:50%;height:50%; padding:7px; background-color:white;"/>
            </div>
            <p></p>
            <div>
                <button {onclick} class="w3-btn w3-blue">{ "Click to Get a New Flag" }</button>
            </div>
        </div>
        </main>
    }
}
