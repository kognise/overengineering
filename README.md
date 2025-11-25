![overengineeRING](https://raw.githubusercontent.com/kognise/overengineering/main/banner.png)

a [webring](https://en.wikipedia.org/wiki/Webring) of interesting people; makers of technology, music, art, or writing.

everyone on this list has different skill levels and different personalities, but i guarantee you'll get something out of talking to them or looking at their sites.

<https://overengineering.kognise.dev/>

## joining

do you make things and have a website showcasing such things? you should join!

[create a pull request](https://github.com/kognise/overengineering/new/main?filename=members/your_name_here.yaml&value=%23%20make%20sure%20to%20change%20the%20filename%20to%20your_name%2Eyaml%20%28alphanumeric%20with%20underscores%29%0A%23%20and%20delete%20this%20comment%21%0A%23%0A%23%20excited%20to%20have%20you%20join%20overengineeRING%20%3A%29%0A%0Aname%3A%20your%20name%20here%0Aurl%3A%20https%3A%2F%2Fexample%2Ecom%2F%0A%0A%23%20%3D%3D%3D%3D%20optional%20settings%3A%20%3D%3D%3D%3D%0A%23%20colors%3A%0A%23%20%20%20border%3A%20%27%23000000%27%0A%23%20%20%20text%3A%20%27%23000000%27%0A%23%20%20%20links%3A%20%27%230000ee%27%0A%23%20stylesheets%3A%0A%23%20%20%20-%20https%3A%2F%2Ffonts%2Egoogleapis%2Ecom%2Fcss2%3Ffamily%3DIBM%2BPlex%2BMono%3Awght%40400%26display%3Dswap%0A%23%20font_stack%3A%20%27%22IBM%20Plex%20Mono%22%2C%20monospace%27%0A%23%20font_size%3A%201em) adding a config file for your site. in the body, say a bit about why you want to join and include a link to your website. make sure you follow the criteria outlined below. you can also shoot an email to [hi@kognise.dev](mailto:hi@kognise.dev).

as soon as you're added the webring, you can add the embed to your site. it will work immediately for your site's visitors, and you will show up on every other site as soon as the automated healthcheck script succeeds.

a full config file looks like this, everything besides `name` and `url` is optional:

```yaml
name: kognise
url: https://kognise.dev/
colors:
  border: '#ced4da'
  text: '#000000'
  links: '#6741d9'
stylesheets:
  - https://fonts.googleapis.com/css2?family=IBM+Plex+Mono:wght@400&display=swap
font_stack: '"IBM Plex Mono", monospace'
font_size: 1em
```

and renders like this:

![kognise.dev embed example](https://doggo.ninja/h0cO3D.png)

`colors`, `stylesheets`, `font_size`, and `font_stack` can be provided if you wish to make the embed's design more closely match your site.

the link color should have good contrast with your website background and white.

## embed

to embed the webring on your site, replace `<slug>` with the part of your config file name before `.yaml`:

```html
<iframe src='https://overengineering.kognise.dev/embed/<slug>' title='overengineeRING embed' width='100%' height='100' style='user-select: none;' frameBorder='0'></iframe>
```

`text_color`, `border_color`, `link_color`, and `on_link_color` can be specified as query parameters and will override the colors specified in your config entry. colors in your config entry are optional and will, in turn, override the root defaults.

## criteria

- this is a webring containing personal sites only.
- you should be an interesting person! a great gauge is whether you think people will get something out of visiting your website, whether inspiration or curiosity.
- members must embed the webring widget on the main page of their site.
- no illegal, nsfw, or gory content is allowed. duh.
- don't be evil, unless you really have to.
