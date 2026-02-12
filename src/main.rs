#[macro_use]
extern crate rocket;
use chrono::{Datelike, Duration, Local, Utc};
use diesel::{Connection, RunQueryDsl, SqliteConnection};
use lazy_static::lazy_static;
use overengineering::config::Member;
use overengineering::health::{Health, MemberManager};
use overengineering::models::{NewHit, SiteStats};
use overengineering::schema::hits;
use rand::seq::SliceRandom;
use rocket::http::Method;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::response::content::{RawHtml, RawJson};
use rocket::response::Redirect;
use rocket::shield::Shield;
use rocket::tokio::sync::Mutex;
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
use sha2::{Digest, Sha256};
use std::net::IpAddr;
use std::{convert::Infallible, future::Future, pin::Pin};

lazy_static! {
    static ref MEMBER_MANAGER: MemberManager = MemberManager::new();
    static ref DB: Mutex<SqliteConnection> =
        Mutex::new(SqliteConnection::establish("./stats.db").unwrap());
}

fn html(mut markup: String) -> RawHtml<String> {
    RawHtml(
        minify_html_onepass::in_place_str(
            &mut markup,
            &minify_html_onepass::Cfg {
                minify_css: true,
                minify_js: false,
            },
        )
        .unwrap()
        .to_string(),
    )
}

#[derive(Debug)]
struct LastSegment(Option<String>);

impl<'r> FromRequest<'r> for LastSegment {
    type Error = Infallible;
    fn from_request<'a: 't, 't>(
        request: &'r Request<'a>,
    ) -> Pin<Box<dyn Future<Output = Outcome<Self, Self::Error>> + Send + 't>> {
        Outcome::Success(LastSegment(
            request
                .headers()
                .get_one("Referer")
                .and_then(|h| h.split("/").last())
                .map(|s| s.to_string()),
        ))
        .pin()
    }
}

#[get("/")]
async fn index() -> RawHtml<String> {
    let mut ok_members: Vec<Member> = vec![];
    let mut not_ok_members: Vec<(Member, Option<Health>)> = vec![];

    for (member, health) in MEMBER_MANAGER.members().await {
        if matches!(health, Some(Health::Ok)) {
            ok_members.push(member);
        } else {
            not_ok_members.push((member, health));
        }
    }

    html(format!(
        "
            <!DOCTYPE html>
            <html lang='en'>
                <head>
                    <meta charset='utf-8'>
                    <meta name='viewport' content='width=device-width, initial-scale=1.0'>
                    <title>overengineeRING</title>
                    <style>
                        body {{
                            background: #0b1728;
                            color: #bdd2ff;
                            font-family: ui-monospace, Menlo, Consolas, Monaco, Liberation Mono, Lucida Console, monospace;
                            margin: 0;
                            padding: 40px;
                            line-height: 1.4;
                            box-sizing: border-box;
                            font-size: 0.8125rem;
                        }}
                        img {{
                            width: min(100%, 1200px);
                            height: auto;
                            margin: 0 auto;
                            display: block;
                        }}
                        h1 {{ margin: 0; }}
                        main {{ max-width: 800px; margin: 0 auto; }}
                        h2 {{ margin: 0; margin-top: 30px; }}
                        .banner {{
                            font-size: 1.25rem;
                            color: #60a5ff;
                            margin-top: 0;
                            margin-bottom: 20px;
                        }}
                        p, ul, ol {{ margin: 10px 0; }}
                        a {{ color: #ff6b60; }}
                        .dim {{ color: #4a6294; }}
                        .failures {{ font-size: 0.9em; opacity: 0.5; }}
                        table {{
                            border-collapse: collapse;
                            overflow: auto;
                            display: block;
                            padding-left: 40px;
                        }}
                        th, td {{
                            padding: 5px 10px;
                            text-align: left;
                        }}
                        th:first-child, td:first-child {{
                            padding-left: 0;
                        }}
                        thead {{
                            border-bottom: 1px solid #4a6294;
                        }}
                        th:not(:last-child), td:not(:last-child) {{
                            border-right: 1px solid #4a6294;
                        }}
                        ::marker {{ color: #4a6294; }}
                        ::selection {{ background: #9d1f15; color: #ffffff; }}
                        @media (max-width: 500px) {{
                            body {{ padding: 20px; }}
                        }}
                    </style>
                </head>
                <body>
                    <h1>
                        <img src='https://raw.githubusercontent.com/kognise/overengineering/main/banner.png' alt='overengineeRING logo' title='overengineeRING' width='1200' height='200'>
                    </h1>

                    <main>
                        <p class='banner'>overengineeRING 2: now with healthchecks, cooler people, and seasonal themes!</p>

                        <p>a <a href='https://en.wikipedia.org/wiki/Webring' target='_blank' rel='noopener noreferrer'>webring</a> of interesting people; makers of technology, music, art, or writing. (<a href='https://github.com/kognise/overengineering/' target='_blank' rel='noopener noreferrer'>github</a>)</p>
                        <p>everyone on this list has different skill levels and different personalities, but i guarantee you'll get something out of talking to them or looking at their sites.</p>

                        <h2>alive members</h2>
                        <p>the ring order is randomized and changes daily! current ordering:</p>
                        <ol>{ok_member_list}</ol>

                        <h2>criteria</h2>
                        <ul>
                            <li>this is a webring containing personal sites only.</li>
                            <li>you should be an interesting person! a great gauge is whether you think people will get something out of visiting your website, whether inspiration or curiosity.</li>
                            <li>no illegal, nsfw, or gory content is allowed. duh.</li>
                            <li>members must embed the webring widget on the homepage of their site.</li>
                            <li>don't be evil, unless you really have to.</li>
                        </ul>
                        <p>do you make things and have a website showcasing such things? you should join! email <a href='mailto:hi@kognise.dev' target='_blank' rel='noopener noreferrer'>hi@kognise.dev</a> asking politely, or directly <a href='https://github.com/kognise/overengineering/new/main?filename=members/your_name_here.yaml&value=%23%20make%20sure%20to%20change%20the%20filename%20to%20your%5Fname%2Eyaml%20%28alphanumeric%20with%20underscores%29%0A%23%20and%20delete%20the%20comments%21%0A%23%0A%23%20excited%20to%20have%20you%20join%20overengineeRING%20%3A%29%0A%0Aname%3A%20your%20name%20here%0Aurl%3A%20https%3A%2F%2Fexample%2Ecom%2F%0A%0A%23%20%3D%3D%3D%3D%20optional%20settings%3A%20%3D%3D%3D%3D%0A%23%20colors%3A%0A%23%20%20%20border%3A%20%27%23000000%27%0A%23%20%20%20text%3A%20%27%23000000%27%0A%23%20%20%20links%3A%20%27%230000ee%27%0A%23%20%20%20on%5Flinks%3A%20%27%23ffffff%27%0A%23%20stylesheets%3A%0A%23%20%20%20%2D%20https%3A%2F%2Ffonts%2Egoogleapis%2Ecom%2Fcss2%3Ffamily%3DIBM%2BPlex%2BMono%3Awght%40400%26display%3Dswap%0A%23%20font%5Fstack%3A%20%27%22IBM%20Plex%20Mono%22%2C%20monospace%27%0A%23%20font%5Fsize%3A%201%2E2em' target='_blank' rel='noopener noreferrer'>create a pull request</a> adding your config file.
                        
                        <div class='failures'>
                            <h2>healthcheck failures</h2>
                            <p>members who fail their healthchecks will not show up on webring member sites or the random button.</p>
                            <p>(the plurality of dead members is partially caused by some serious downtime over the past couple of years. v2 fixes this!)</p>
                            <table>
                                <thead>
                                    <tr>
                                        <th>name</th>
                                        <th>failure reason</th>
                                        <th>url</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {not_ok_member_list}
                                </tbody>
                            </table>
                        </div>
                    </main>
                </body>
            </html>
        ",
        ok_member_list = ok_members.into_iter()
            .map(|member| format!("
                <li>
                    <div><a href='{url}' target='_blank' rel='noopener noreferrer'>{name}</a></div>
                    <div class='dim'>{url}</div>
                </li>", url = member.url, name = member.name))
            .collect::<Vec<String>>()
            .join(""),
        not_ok_member_list = not_ok_members.into_iter()
            .map(|(member, health)| format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td></tr>",
                member.name,
                match health {
                    Some(Health::Ok) => unreachable!(),
                    Some(Health::SiteUnreachable) => "site unreachable",
                    Some(Health::NoWebringEmbed) => "embed missing from site",
                    Some(Health::SlugMismatch(_)) => "embed url has wrong slug",
                    None => "healthcheck pending...",
                },
                member.url,
            ))
            .collect::<Vec<String>>()
            .join("")
    ))
}

#[get("/rand")]
async fn random(last_segment: LastSegment) -> Redirect {
    Redirect::to(
        MEMBER_MANAGER
            .members()
            .await
            .into_iter()
            .filter_map(|(member, health)| {
                if matches!(health, Some(Health::Ok)) && last_segment.0 != Some(member.slug) {
                    Some(member.url)
                } else {
                    None
                }
            })
            .collect::<Vec<String>>()
            .choose(&mut rand::thread_rng())
            .cloned()
            .unwrap(),
    )
}

#[get("/embed/<slug>?<text_color>&<border_color>&<link_color>&<on_link_color>&<font_size>")]
async fn embed(
    ip: IpAddr,
    slug: &str,
    text_color: Option<&str>,
    border_color: Option<&str>,
    link_color: Option<&str>,
    on_link_color: Option<&str>,
    font_size: Option<&str>,
) -> RawHtml<String> {
    // Healthy members, and this site!
    let members: Vec<Member> = MEMBER_MANAGER
        .members()
        .await
        .into_iter()
        .filter_map(|(member, health)| {
            if matches!(health, Some(Health::Ok)) || member.slug == slug {
                Some(member)
            } else {
                None
            }
        })
        .collect();

    let (member_index, member) = members
        .iter()
        .enumerate()
        .find(|(_, site)| site.slug == slug)
        .unwrap();

    {
        let ip_octets = match ip.to_canonical() {
            IpAddr::V4(ip) => ip.octets().to_vec(),
            IpAddr::V6(ip) => ip.octets().to_vec(),
        };
        let ip_hash = Sha256::digest(ip_octets);
        let ip_hash = ip_hash.as_slice();
        let new_hit = NewHit {
            slug: &slug,
            timestamp: Utc::now(),
            ip_hash,
        };
        let mut db = DB.lock().await;
        diesel::insert_into(hits::table)
            .values(&new_hit)
            .execute(&mut *db)
            .unwrap();
    }

    html(format!(
        "
            <!DOCTYPE html>
            <html lang='en'>
                <head>
                    <meta charset='utf-8'>
                    <meta name='robots' content='noindex'>
                    <title>overengineeRING embed</title>
                    <style>
                        body {{
                            font-family: {font_stack};
                            font-size: {font_size};
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
                        a:hover {{ color: {on_link_color}; background: {link_color}; }}
                        a::before {{ content: '['; }}
                        a::after {{ content: ']'; }}
                        p {{ margin: 0 0 10px 0; }}
                    </style>
                    {head_include}
                </head>
                <body>
                    <a href='{prev_url}' target='_parent'>&laquo; prev</a>
                    <nav>
                        <p>{name} @ overengineeRING</p>
                        <div>
                            <a href='/' target='_blank'>list</a>
                            <a href='/rand' target='_parent'>rand</a>
                        </div>
                    </nav>
                    <a href='{next_url}' target='_parent'>next &raquo;</a>
                    <script>
                        {theme_js}
                    </script>
                </body>
            </html>
        ",
        name = member.name,
        prev_url = members[if member_index == 0 { members.len() - 1 } else { member_index - 1 }].url,
        next_url = members[(member_index + 1) % members.len()].url,
        font_stack = member.font_stack.as_ref().unwrap_or(&"monospace".to_string()),
        font_size = font_size.unwrap_or(member.font_size.as_ref().unwrap_or(&"initial".to_string())),
        head_include = member.stylesheets.iter()
            .map(|stylesheet| format!("<link rel='stylesheet' href='{}'>", stylesheet))
            .collect::<Vec<String>>()
            .join(""),
        text_color = text_color.as_ref().unwrap_or(&member.colors.text.as_str()),
        border_color = border_color.as_ref().unwrap_or(&member.colors.border.as_str()),
        link_color = link_color.as_ref().unwrap_or(&member.colors.links.as_str()),
        on_link_color = on_link_color.as_ref().unwrap_or(&member.colors.on_links.as_str()),
        theme_js = get_theme_js(),
    ))
}

fn get_theme_js() -> &'static str {
    let pacific_time_ish = Local::now() - Duration::hours(8);
    let month = pacific_time_ish.month();
    let day = pacific_time_ish.day();

    match (month, day) {
        (2, 14) => include_str!("seasonal/valentines.js"),
        _ => "",
    }
}

fn num_fmt(num: i32) -> String {
    num.to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join(",")
}

#[get("/stats")]
async fn stats() -> RawHtml<String> {
    let members: Vec<Member> = MEMBER_MANAGER
        .members()
        .await
        .into_iter()
        .filter_map(|(member, health)| {
            if matches!(health, Some(Health::Ok)) {
                Some(member)
            } else {
                None
            }
        })
        .collect();
    let stats = {
        let mut db = DB.lock().await;
        SiteStats::fetch(&mut db).unwrap()
    };

    let mut visitors_sum = 0;
    let mut from_others_sum = 0;
    for member in &members {
        let default = SiteStats::default_for_slug(&member.slug);
        let stats = stats
            .iter()
            .find(|stats| stats.slug == member.slug)
            .unwrap_or(&default);

        visitors_sum += stats.total_unique_visitors;
        from_others_sum += stats.returning_users;
    }

    html(format!(
        "
            <!DOCTYPE html>
            <html lang='en'>
                <head>
                    <meta charset='utf-8'>
                    <meta name='viewport' content='width=device-width, initial-scale=1.0'>
                    <title>overengineeRING</title>
                    <style>
                        body {{
                            background: #0b1728;
                            color: #bdd2ff;
                            font-family: ui-monospace, Menlo, Consolas, Monaco, Liberation Mono, Lucida Console, monospace;
                            margin: 0;
                            padding: 20px;
                            line-height: 1.4;
                            box-sizing: border-box;
                            font-size: 0.8125rem;
                        }}
                        h1 {{ margin: 0; }}
                        p {{ margin: 10px 0; }}
                        a {{ color: #ff6b60; }}
                        table {{
                            border-collapse: collapse;
                            overflow: auto;
                            display: block;
                        }}
                        th, td {{
                            padding: 5px 10px;
                            text-align: left;
                        }}
                        td {{
                            text-align: right;
                        }}
                        th:first-child, td:first-child {{
                            padding-left: 0;
                        }}
                        thead {{
                            border-bottom: 1px solid #4a6294;
                        }}
                        th:not(:last-child), td:not(:last-child) {{
                            border-right: 1px solid #4a6294;
                        }}
                        ::selection {{ background: #9d1f15; color: #ffffff; }}
                    </style>
                </head>
                <body>
                    <h1>stats (beta)</h1>
                    <p><a href='/'>go home</a></p>
                    <p>overengineeRING drove {driven_pct}% of the new visitors to members' websites!</p>
                    <table>
                        <thead>
                            <tr>
                                <th>slug</th>
                                <th>visitors</th>
                                <th>from others</th>
                                <th>first site</th>
                                <th>-> others</th>
                            </tr>
                        </thead>
                        <tbody>
                            {stats_list}
                        </tbody>
                    </table>
                </body>
            </html>
        ",
        stats_list = members
            .into_iter()
            .map(|member| {
                let default = SiteStats::default_for_slug(&member.slug);
                let stats = stats
                    .iter()
                    .find(|stats| stats.slug == member.slug)
                    .unwrap_or(&default);

                format!(
                    "<tr>
                        <th>{}</th>
                        <td>{}</td>
                        <td>{}</td>
                        <td>{}</td>
                        <td>{}</td>
                    </tr>",
                    member.slug,
                    num_fmt(stats.total_unique_visitors),
                    num_fmt(stats.returning_users),
                    num_fmt(stats.first_visit_users),
                    num_fmt(stats.driven_to_others),
                )
            })
            .collect::<Vec<String>>()
            .join(""),
        driven_pct = ((from_others_sum as f32 / visitors_sum as f32) * 100.0).round() as u32,
    ))
}

#[get("/members.json")]
async fn members() -> RawJson<String> {
    RawJson(serde_json::to_string(&MEMBER_MANAGER.members().await).unwrap())
}

#[launch]
async fn rocket() -> _ {
    let _ = MEMBER_MANAGER.members().await;

    let cors = CorsOptions {
        allowed_origins: AllowedOrigins::All,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::All,
        allow_credentials: false,
        ..Default::default()
    }
    .to_cors()
    .expect("couldn't create cors options");

    rocket::build()
        .attach(Shield::new())
        .mount("/", routes![index, random, embed, members, stats])
        .attach(cors)
}
