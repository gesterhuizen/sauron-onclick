Rust code illustrating unexpected behaviour (to the original author) in how the `onclick` handler in a `button` type input is updated. It is a modified version of the minimal Sauron example: https://github.com/ivanceras/sauron/tree/master/examples/minimal.

# Steps to reproduce
1. Clone this repo: https://github.com/gesterhuizen/sauron-onclick
2. Build it:
```
. ./bootstrap.sh
make build server
```
3. Open the app in the browser: http://localhost:4001/
4. Click the button twice.

# Expected
1. After two button clicks, the `VIEW_COUNT` on the button and in the text field is `3`. This is expected because the button value and text field reflects the value that `VIEW_COUNT` held at the time that the view was rendered.
2. After two button clicks, the `number` field is `3`. This is expected because, when the button is clicked, the `number` field is updated with the value that `VIEW_COUNT` held at the time that the view was rendered. We assign a new `onclick` closure each time the view is rendered (https://github.com/gesterhuizen/sauron-onclick/blob/master/src/lib.rs#L40):
```
    fn view(&self) -> Node<Msg> {
        VIEW_COUNT.fetch_add(1, Ordering::SeqCst);
        let vc = VIEW_COUNT.load(Ordering::SeqCst);

...
                    [input(
                        [
                            r#type("button"),
                            value(format!("VIEW_COUNT: {}", vc)),
                            onclick(move |_| {
                                sauron::log(format!("Button is clicked (VIEW_COUNT = {})", vc));
                                Msg::Click(vc)
                            }),
                        ],
                        [],
                    )],
```

# Actual
1. Expected: After two button clicks, `VIEW_COUNT` on the button and in the text field is `3`.
2. Unexpected: After two button clicks, the `number` field displays `1` (and stays in this state, no matter how many times the button is clicked). It seems like first `onclick` handler that was set (i.e. when `VIEW_COUNT` held the value `1`) is used even after multiple view rerenders.
