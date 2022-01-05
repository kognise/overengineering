#[macro_use]
extern crate rocket;
use lazy_static::lazy_static;
use rocket::response::content::Html;
use rocket::shield::Shield;
use rocket::response::Redirect;
use serde::{Deserialize, Serialize};
use rand::seq::SliceRandom;

const CONFIG_YAML: &str = include_str!("../config.yaml");

lazy_static! {
    static ref CONFIG: Config = serde_yaml::from_str(&CONFIG_YAML).unwrap();
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SiteColors {
    pub text: String,
    pub border: String,
    pub links: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Site {
    pub name: String,
    pub url: String,
    pub colors: Option<SiteColors>,
    pub font_stack: Option<String>,
    pub stylesheets: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub sites: Vec<Site>,
    pub default_colors: SiteColors,
}

#[inline(always)]
fn html(mut markup: String) -> Html<String> {
    Html(minify_html_onepass::in_place_str(&mut markup, &minify_html_onepass::Cfg { minify_css: true, minify_js: false }).unwrap().to_string())
}

#[get("/")]
fn index() -> Html<String> {
    html(format!(
        "
            <!DOCTYPE html>
            <html lang='en'>
                <head>
                    <meta charset='utf-8'>
                    <meta name='viewport' content='width=device-width'>
                    <title>overengineeRING</title>
                    <style>
                        body {{
                            font-family: monospace;
                            margin: 0;
                            padding: 30px;
                            box-sizing: border-box;
                            font-size: 1.1em;
                        }}
                        h1 {{ margin: 0; }}
                        h2 {{ margin: 0; margin-top: 30px; }}
                        p, ul {{ margin: 10px 0; }}
                    </style>
                </head>
                <body>
                    <h1>overengineeRING</h1>
                    <p>a <a href='https://en.wikipedia.org/wiki/Webring' target='_blank' rel='noopener noreferrer'>webring</a> of people who make cool stuff.</p>

                    <h2>members</h2>
                    <ul>{member_list}</ul>

                    <h2>criteria</h2>
                    <ul>
                        <li>this is a webring containing personal sites only.</li>
                        <li>sites must list at least 3 things the owner made or majorly contributed to.</li>
                        <li>no illegal, nsfw, or gory content is allowed. duh.</li>
                        <li>members must embed the webring widget on the main page of their site.</li>
                        <li>don't be evil, unless you really have to.</li>
                    </ul>
                    <p>wanna join? <a href='https://github.com/kognise/overengineering/' target='_blank' rel='noopener noreferrer'>head over to github</a> and create a pull request, or contact <a href='https://kognise.dev/' target='_blank' rel='noopener noreferrer'>me</a> asking politely.
                </body>
            </html>
        ",
        member_list = CONFIG.sites.iter()
            .map(|s| format!("<li><a href='{}' target='_blank' rel='noopener noreferrer'>{}</a></li>", s.url, s.name))
            .collect::<Vec<String>>()
            .join("")
    ))
}

#[get("/rand")]
fn random() -> Redirect {
    Redirect::to(&CONFIG.sites.choose(&mut rand::thread_rng()).unwrap().url)
}

#[get("/embed/<name>?<text_color>&<border_color>&<link_color>")]
fn embed(name: String, text_color: Option<String>, border_color: Option<String>, link_color: Option<String>) -> Html<String> {
    let site_index = CONFIG
        .sites
        .iter()
        .position(|site| site.name == name)
        .unwrap();
    let ref site = CONFIG.sites[site_index];

    let mut colors = site.colors.as_ref().unwrap_or(&CONFIG.default_colors).clone();
    if let Some(text_color) = text_color {
        colors.text = text_color;
    }
    if let Some(border_color) = border_color {
        colors.border = border_color;
    }
    if let Some(link_color) = link_color {
        colors.links = link_color;
    }

    html(format!(
        "
            <!DOCTYPE html>
            <html lang='en'>
                <head>
                    <meta charset='utf-8'>
                    <title>overengineeRING embed</title>
                    <style>
                        body {{
                            font-family: {font_stack};
                            color: {text_color};
                            border: 1px solid {border_color};
                            margin: 0;
                            padding: 20px;
                            box-sizing: border-box;
                            display: flex;
                            flex-direction: row;
                            align-items: center;
                            text-align: center;
                            user-select: none;
                        }}
                        html, body {{ background: transparent; height: 100%; overflow: hidden; }}
                        nav {{ flex: 1; }}
                        div {{ display: flex; flex-direction: row; gap: 14px; justify-content: center; }}
                        a {{ display: inline-block; padding: 1px; color: {link_color}; text-decoration: none; }}
                        a:hover {{ color: #ffffff; background: {link_color}; }}
                        a::before {{ content: '['; }}
                        a::after {{ content: ']'; }}
                        p {{ margin: 0 0 10px 0;}}
                    </style>
                    {head_include}
                </head>
                <body>
                    <a href='{prev_url}' target='_blank'>&laquo; prev</a>
                    <nav>
                        <p>{name} @ overengineeRING</p>
                        <div>
                            <a href='/' target='_blank'>list</a>
                            <a href='/rand' target='_blank'>rand</a>
                        </div>
                    </nav>
                    <a href='{next_url}' target='_blank'>next &raquo;</a>
                </body>
            </html>
        ",
        name = name,
        prev_url = CONFIG.sites[if site_index == 0 { CONFIG.sites.len() - 1 } else { site_index - 1 }].url,
        next_url = CONFIG.sites[(site_index + 1) % CONFIG.sites.len()].url,
        font_stack = site.font_stack.as_ref().unwrap_or(&"monospace".to_string()),
        head_include = site.stylesheets.as_ref().map(
            |stylesheets| stylesheets.iter()
                .map(|stylesheet| format!("<link rel='stylesheet' href='{}'>", stylesheet))
                .collect::<Vec<String>>().join("")
        ).unwrap_or("".to_string()),
        text_color = colors.text,
        border_color = colors.border,
        link_color = colors.links,
    ))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Shield::new())
        .mount("/", routes![index, random, embed])
}
