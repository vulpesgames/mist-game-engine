use mist_dom::render;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SenkoProps {
    hello: String,
    senko: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TextProps {
    content: String,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct EepProps {
}

#[derive(Clone, Debug)]
pub struct Senko {
    counter: i32,
    props: SenkoProps,
}

#[derive(Clone, Debug)]
pub struct Eep {
    props: TextProps,
}

fn main() {
    render! {
        <Senko hello="uyan" senko={3 * 2}>
            <Eep />
            <Text value="わらわは神使のキツネなのじゃ！"/>
        </Senko>
    };

    /*
    // これを
    let e = render! {
        <Senko hello="uyan" senko={3 * 2}>
            <Eep />
            <Text value="わらわは神使のキツネなのじゃ！"/>
        </Senko>
    };

    // こうしたい
    let e_1 = {
        let mut __0 = Senko::create_element(<Senko as Component>::Props {
            hello: "うやん",
            senko: {3 * 2},
            ..Default::default()
        });

        {
            let mut __c_1 = Collection::new();
            let mut __2 = Eep::create_element(<Eep as Component>::Props {..Default::default()});
            let mut __3 = Text::create_element(<Text as Component>::Props {
                value: "わらわは神使のキツネなのじゃ！",
                ..Default::default()
            });

            __c_1.append(__2);
            __c_1.append(__3);

            __c_1.insert_to(&mut __0);
        }

        __0
    };

    assert_eq!(e, e1);
    */
}
