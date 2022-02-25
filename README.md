![overengineeRING](https://raw.githubusercontent.com/kognise/overengineering/main/banner.png)

a [webring](https://en.wikipedia.org/wiki/Webring) of interesting people; makers of technology, music, art, or writing.

everyone on this list has different skill levels and different personalities, but i guarantee you'll get something out of talking to them or looking at their sites.

<https://overengineering.kognise.dev/>

## joining

do you make things and have a website showcasing such things? you should join!

[create a pull request](https://github.com/kognise/overengineering/edit/main/config.yaml) adding yourself to the config file. in the body, say a bit about why you want to join and include a link to your website. make sure you follow the criteria outlined below. you can also shoot an email to [hi@kognise.dev](mailto:hi@kognise.dev).

a full config entry looks like this, everything besides `name` and `url` is optional:

```yaml
- name: kognise
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

to embed the webring on your site, replace `<name>` with your site's name, of course:

```html
<iframe src='https://overengineering.kognise.dev/embed/<name>' width='100%' height='100' style='user-select: none;' frameBorder='0'></iframe>
```

`text_color`, `border_color`, and `link_color` can be specified as query parameters and will override the colors specified in your config entry. colors in your config entry are optional and will, in turn, override the root defaults.

## criteria

- this is a webring containing personal sites only.
- you should be an interesting person! a great gauge is whether you think people will get something out of visiting your website, whether inspiration or curiosity.
- members must embed the webring widget on the main page of their site.
- no illegal, nsfw, or gory content is allowed. duh.
- don't be evil, unless you really have to.

do you make things and have a website showcasing such things? you should join!