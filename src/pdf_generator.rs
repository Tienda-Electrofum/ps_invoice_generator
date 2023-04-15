use indoc::indoc;
use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

pub struct PdfGenerator {}

impl PdfGenerator {
    pub fn generate_pdf_test() {

        let (doc, page1, layer1) =
            PdfDocument::new("Invoice", Mm(210.0), Mm(297.0), "Layer 1");
        let font = doc.add_builtin_font(BuiltinFont::TimesBold).unwrap();
        let current_layer = doc.get_page(page1).get_layer(layer1);

        current_layer.set_font(&font, 14.0);
        current_layer.set_text_cursor(Mm(10.0), Mm(267.0));
        current_layer.set_line_height(46.0);
        current_layer.write_text("Electrofum", &font);
        current_layer.add_line_break();
        current_layer.write_text("Carlos Garcia", &font);
        current_layer.add_line_break();

        current_layer.set_text_cursor(Mm(10.0), Mm(100.0));

        Self::put_legal_data(&current_layer, &font);


        current_layer.end_text_section();
        doc.save(&mut BufWriter::new(
            File::create("test_working.pdf").unwrap(),
        ))
        .unwrap();

    }

    fn put_legal_data(current_layer: &PdfLayerReference, font: &IndirectFontRef) {
        const COL1: f64 = 100.0;
        const ROW3: f64 = 10.0;

        //current_layer.set_font(font, 10.0);
        current_layer.set_text_cursor(Mm(ROW3), Mm(COL1));
        current_layer.write_text("text", font);
        

        let legal_text = indoc! {r#"
            TÉRMINOS DE PROTECCIÓN DE DATOS
            De conformidad con el RGPD y la LOPDGDD, en ELECTROFUM en calle
            Calle Girona, 86 - C.P. 08009 Barcelona, tratamos la información que usted
            nos proporciona con la finalidad de gestión de clientes, a nivel administrativo,
            contable y fiscal.
            Los datos no serán cedidos ni comunicados a terceros, salvo en los supuestos
            legalmente establecidos.
            Ud. podrá ejercitar los derechos de Acceso, Rectificación, Cancelación,
            Portabilidad o Supresión. Para ejercitar los derechos deberá presentar un
            escrito en la dirección arriba señalada o mediante correo electrónico a:
            tiendaelectrofum@hotmail.com.
        "#};
        let lines: Vec<_> = legal_text.lines().collect();
    }
}