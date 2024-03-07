#[cfg(test)]

mod tests {
    use crate::recipe::Recipe;

    // Big crime
    // #[test]
    // fn coop_se() {
    //     Recipe::from_url("https://www.coop.se/recept/blabarsmuffins-med-topping/");
    // }

    #[test]
    fn recepten_se() {
        let recipe = Recipe::from_url("https://www.recepten.se/recept/kladdkaka_kolakraem.html");
        println!("{recipe:#?}");
    }

    #[test]
    fn ica_se() {
        Recipe::from_url("https://www.ica.se/recept/fjallroding-med-dill-och-romsas-721604/");
    }

    #[test]
    fn tasteline_com() {
        Recipe::from_url("https://www.tasteline.com/recept/kladdkaka-grande/");
    }

    #[test]
    fn koket_se() {
        Recipe::from_url("https://www.koket.se/enkel-zucchinipasta-med-kryddgront");
    }

    #[test]
    fn recept_se() {
        Recipe::from_url("https://recept.se/recept/vinbarsbal-med-nektariner/");
    }

    #[test]
    fn arla_se() {
        Recipe::from_url("https://www.arla.se/recept/korv-stroganoff/");
    }

    #[test]
    fn expressen_se_allt_om_mat() {
        Recipe::from_url(
            "https://www.expressen.se/alltommat/recept/halstrad-lax-med-brynt-pepparrotsmor/",
        );
    }

    #[test]
    fn svenskarecept_se() {
        Recipe::from_url("https://svenskarecept.se/zucchiniplattar-2/");
    }

    #[test]
    fn fjallbrynt_se() {
        Recipe::from_url(
            "https://www.fjallbrynt.se/recept/ragmacka-med-spicy-salmon-potatis-agg-och-rodbetor/",
        );
    }

    #[test]
    fn viktvaktarna_se() {
        Recipe::from_url("https://www.viktvaktarna.se/se/recept/kycklingfile-med-vitkalsgratang/5847789256a72e1b163c98b7");
    }
}
