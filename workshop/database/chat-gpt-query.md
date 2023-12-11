Scrivi la lista delle entità per la progettazione concettuale nel template
```
#### Nome entità
- campo dati TIPO [|PK] [|NOT NULL]
```
Sostituendo a TIPO il tipo della variabile opportuno in SQL. Non riportare le
relazioni in questo elenco. [value1|value2] vuol dire che devi scegliere tra il
valore1 e il valore2. Nota che ```[|PK]``` vuol dire che il valore in questione
non è una primary key oppure è una primary key. NOT NULL vuol dire che il valore
in questione deve essere definito.

Dopo l'elenco delle entità scrivi la tabella delle relazioni, usa i collegamenti
per elaborare la tabella seguente:

```
| Relazione | Entità coinvolte | Descrizione | Attributi |
```

Nota che il nome della relazione deve essere diverso dai nomi delle entità.
La sezione entità coinvolte deve essere descitta nel modo 

```
Entità1 (min_value1, max_value1)
...
EntitàN (min_valueN, max_valueN)
```

Considerando che per esempio

```
Entità1 (0,1)
Entità2 (1, N)
```

Vuol dire che l'entità 1 è in relazione al più con un'istanza dell'entità 2.
Mentre l'entità 2 deve essere in relazione almeno con un istanza dell'entità 1.
Esistono anche i valori

```
(1, 1)
(N, N)
(0, N)
```
Nota che min_value <= max_value
A partire dalle seguenti descrizioni:

```
#### Cliente
Rappresenta l'utente che può prenotarsi in un ristorante ed effettuare un
ordine. Ogni utente è caratterizzato dai seguenti dati:

- email
- nome utente
- collegamenti:
    - prenotazione
    - ordinazione
    - allergene

#### Ristoratore
Rappresenta l'utente che gestisce un ristorante. Può modificare le informazioni
relative al proprio ristorante e può accettare o rifiutare una prenotazione.
Ogni ristoratore è caratterizzato dai seguenti dati:

- email
- nome utente
- collegamenti:
    - ristorante

#### Ristorante
Rappresenta un ristorante in cui i clienti possono prenotarsi.
Ogni ristorante è caratterizzato dai seguenti dati:

- nome
- orario
- descrizione
- indirizzo
- recapiti
- website
- costo
- sedie per bambini
- adatto a persona con ridotta mobilità
- collegamenti:
    - ristoratore
    - tag

#### Prenotazione
Rappresenta la prenotazione in un ristorante. Un cliente può effettuare una
prenotazione presso un ristorante. Un ristoratore può gestire le prenotazioni
inerenti al proprio ristorante.
Ogni prenotazione è caratterizzata dai seguenti dati:

- data
- ora
- numero persone
- stato
- collegamenti:
    - ristorante
    - utente
    - ordinazione

#### Ordinazione
Rappresenta l'elenco delle pietanze riferito ad un'ordinazione.
Ciascun utente che ha una prenotazione, può aggiungere dei piatti ad essa.
Ogni ordinaizone è caratterizzata dai seguenti dati:

- costo parziale
- collegamenti:
    - utente
    - prenotazione
    - piatto

#### Piatto
Rappresenta un piatto ordinabile in un ristorante, dunque inseribile in 
un'ordinazione.
I piatti sono gestiti dal ristoratore. 
Ogni piatto è caratterizzato dai seguenti dati:

- foto
- nome
- descrizione
- prezzo
- collegamenti:
    - ingrediente
    - ristorante

#### Ingrediente
Rappresenta una delle componenti che compone un piatto.
Ogni ingrediente è caratterizzato dai seguenti dati:

- foto
- nome
- descrizione
- prezzo/quantità
- collegamenti:
    - piatto
    - allergene

#### Allergene
Rappresenta un ingrediente che può causare allergie.
Ciascun cliente può indicare le sostanze di cui è allergico. In fase di
ordinazione saranno segnalati i piatti che il cliente non può mangiare
Ogni allergene è caratterizzato dai seguenti dati:

- nome
- descrizione
- foto
- collegamenti:
    - piatto
    - utente
    - ingrediente

#### Tag

- nome
- descrizione
- collegamenti:
    - ristorante
```

Riporta la risposta in markdown in modo che si possa copiare la risposta.

---

-- for ristorante
CREATE TYPE PRIZE AS ENUM ('expensive', 'normal', 'cheap');

-- for prenotazione
CREATE TYPE STATO_ORDINE AS ENUM (
	'da confermare', 
	'in attesa', 
	'in corso', 
	'concluso'
);

CREATE TYPE ALLERGENE AS ENUM (
	'glutine',
	'crostacei',
	'uova',
	'pesce',
	'arachidi',
	'soia',
	'latte',
	'frutta a guscio',
	'sedano',
	'senape',
	'sesamo',
	'lupini',
	'molluschi'
);

-- Tabella degli utenti
CREATE TABLE utente (
    id INT PRIMARY KEY,
    email VARCHAR(255),
    username VARCHAR(100)
);

-- Tabella dei ristoranti
CREATE TABLE ristorante (
    id_utente INT PRIMARY KEY,
    nome VARCHAR(100),
    orario JSON,
    descrizione VARCHAR(511),
    indirizzo VARCHAR(255),
    telefono VARCHAR(10),
    website VARCHAR(255),
    costo PRIZE_ENUM,
    sedie_per_bambini BOOLEAN,
	adatto_ai_disabili BOOLEAN,
    FOREIGN KEY (id_utente) REFERENCES utente(id)
);

-- Tabella delle prenotazioni
CREATE TABLE prenotazione (
    id INT PRIMARY KEY,
    data_e_ora DATETIME,
    numero_persone INT,
    stato VARCHAR(50),
    id_utente INT,
    id_ristorante INT,
    FOREIGN KEY (id_utente) REFERENCES utente(id),
    FOREIGN KEY (id_ristorante) REFERENCES ristorante(id)
);

-- Tabella dei tag
CREATE TABLE tag (
    id INT PRIMARY KEY,
    nome VARCHAR(50),
    descrizione VARCHAR(255)
);

-- Tabella di collegamento tra tag e ristoranti
CREATE TABLE cucina (
    id_tag INT,
    id_ristorante INT,
    FOREIGN KEY (id_tag) REFERENCES tag(id),
    FOREIGN KEY (id_ristorante) REFERENCES ristorante(id)
);

-- Tabella delle ordinazioni
CREATE TABLE ordinazione (
    id_utente INT,
    id_prenotazione INT,
    id_piatto INT,
    quantita INT,
	PRIMARY KEY (id_utente, id_prenotazione, id_piatto),
    FOREIGN KEY (id_utente) REFERENCES utente(id),
    FOREIGN KEY (id_prenotazione) REFERENCES prenotazione(id),
    FOREIGN KEY (id_piatto) REFERENCES piatto(id)
);

-- Tabella dei menu dei ristoranti
CREATE TABLE menu (
    id_ristorante INT,
    id_piatto INT,
    prezzo DECIMAL(10, 2),
	PRIMARY KEY (id_ristorante, id_piatto),
    FOREIGN KEY (id_ristorante) REFERENCES ristorante(id),
    FOREIGN KEY (id_piatto) REFERENCES piatto(id)
);

-- Tabella dei piatti
CREATE TABLE piatto (
    id INT PRIMARY KEY,
    nome VARCHAR(100),
    descrizione VARCHAR(511),
    foto BYTEA
);

-- Tabella di collegamento tra piatti e ingredienti
CREATE TABLE composizione (
    id_piatto INT,
    id_ingrediente INT,
    FOREIGN KEY (id_piatto) REFERENCES piatto(id),
    FOREIGN KEY (id_ingrediente) REFERENCES ingrediente(id)
);

-- Tabella degli ingredienti
CREATE TABLE ingrediente (
    id INT PRIMARY KEY,
    nome VARCHAR(100),
    descrizione VARCHAR(255),
    foto BYTEA
);

-- Tabella degli allergeni
CREATE TABLE allergene (
    id_ingrediente INT,
    allergene ALLERGENE,
    foto BYTEA,
    FOREIGN KEY (id_ingrediente) REFERENCES ingrediente(id)
);

-- Tabella delle allergie degli utenti
CREATE TABLE allergia (
    id_utente INT,
    id_allergene INT,
    FOREIGN KEY (id_utente) REFERENCES utente(id),
    FOREIGN KEY (id_allergene) REFERENCES allergene(id_ingrediente)
);
---
considerando lo schem appena descritto, scrivi le query per effettuare le 
seguenti operazioni: 
- creazione di un utente
- modifica di un utente
- creazione di un ristorante
- modifica di un ristorante
- creazione di una prenotazione
- modifica di una prenotazione
- crezione di un tag
- modifica di un tag
- creazione di una cucina
- rimozione di una cucina
- creazione di un piatto 
- modifica di un piatto
- creazione di un menu
- creazione di un ingrediente
- modifica di un ingrediente
- creazione di una composizione
- rimozione di una composizione
- creazione di un allergene
- modifica di un allergene
- creazione di un'allergia
- rimozione di un allergia
