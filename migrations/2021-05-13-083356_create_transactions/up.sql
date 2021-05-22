-- Your SQL goes here
CREATE TABLE "transactions" (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    booking_date TEXT NOT NULL, -- Buchungstag
    value_date TEXT NOT NULL, -- Wertstellung
    booking_text TEXT NOT NULL, -- Buchungstext
    beneficiary TEXT NOT NULL, -- Auftraggeber / Begünstigter
    purpose TEXT NOT NULL, -- Verwendungszweck
    account_number TEXT NOT NULL, -- Kontonummer
    sort_code TEXT NOT NULL, -- BLZ
    amount TEXT NOT NULL, -- Betrag (EUR)
    creditor_id TEXT NOT NULL, -- Gläubiger-ID
    mandate_reference TEXT NOT NULL, -- Mandatsreferenz
    customer_reference TEXT NOT NULL -- Kundenreferenz
);

INSERT INTO
    "transactions" (booking_date, value_date, booking_text, beneficiary, purpose, account_number, sort_code, amount, creditor_id, mandate_reference, customer_reference)
VALUES
("08.02.2019","08.02.2019","Gutschrift","Test Full Name","test","DE5511199009900990099","DEUTDEDBBER","1,00","","","test"),
("11.02.2019","11.02.2019","Umbuchung","KREDITKARTEN GELDANLAGE","Test Full Name","DE561999888","DEUTDEDBBER","15,00","","",""),
("11.02.2019","11.02.2019","Umbuchung","KREDITKARTEN GELDANLAGE","Test Full Name2","DE561999588","DEUTDEDBBER","10,00","","",""),
("12.02.2019","12.02.2019","Umbuchung","KREDITKARTEN GELDANLAGE","Test Full Name3","DE561999688","DEUTDEDBBER","500,00","","",""),
("25.02.2019","25.02.2019","Umbuchung","KREDITKARTEN GELDANLAGE","Test Full Name4","DE561999788","DEUTDEDBBER","25,00","","","");
