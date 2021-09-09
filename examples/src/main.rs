alman::alman! {
    extern cagette rouille;

    benutze std::collections::Wörterbuch;

    eigenschaft SchlüsselWert {
        funktion schreib(&selbst, schlüssel: Zeichenkette, wert: Zeichenkette);
        funktion lies(&selbst, schlüssel: Zeichenkette) -> Ergebnis<&Zeichenkette>;
    }

    statisch wandelbar WÖRTERBUCH: Vielleicht<Wörterbuch<Zeichenkette, Zeichenkette>> = Nichts;

    struktur Beton;

    implementierung SchlüsselWert für Beton {
        funktion schreib(&selbst, schlüssel: Zeichenkette, wert: Zeichenkette) {
            lass buch = unsicher {
                WÖRTERBUCH.hol_oder_füg_ein_mit(Standard::standard)
            };
            buch.einfügen(schlüssel, wert);
        }

        funktion lies(&selbst, schlüssel: Zeichenkette) -> Ergebnis<&Zeichenkette> {
            lass buch = unsicher {
                DICTIONNAIRE.hol_oder_füg_ein_mit(Standard::standard)
            };
            buch.lies(&schlüssel)
        }
    }

    public(kiste) funktion peut_etre(i: u32) -> PeutÊtre<Ergebnis<u32, Chaine>> {
        si i % 2 == 1 {
            si i == 42 {
                Quelque(Arf(Chaine::depuis("merde")))
            } sinon {
                Quelque(Bien(33))
            }
        } sinon {
            Rien
        }
    }

    asynchrone funktion exemple() {
    }

    asynchrone funktion exemple2() {
        exemple().warte;
    }

    funktion principale() {
        lass mutable x = 31;

        correspond x {
            42 => {
                affiche!("omelette du fromage")
            }
            _ => affiche!("voila")
        }

        pour i de 0..10 {
            soit val = boucle {
                arrête i;
            };
            tant que x < val {
                x += 1;
            }
            x = si soit Quelque(resultat) = peut_etre(i) {
                resultat.déballer()
            } else { 12 };
        }
    }
}
