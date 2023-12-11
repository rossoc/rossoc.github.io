CREATE SCHEMA IF NOT EXISTS easy-meal;
-- for ristorante
IF NOT EXISTS CREATE TYPE PRIZE AS ENUM ('expensive', 'normal', 'cheap');

-- for prenotazione
IF NOT EXISTS CREATE TYPE STATO_ORDINE AS ENUM (
	'da confermare', 
	'in attesa', 
	'in corso', 
	'concluso'
);

IF NOT EXISTS CREATE TYPE ALLERGENE AS ENUM (
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
IF NOT EXISTS CREATE TABLE utente (
    id INT AUTO_INCREMENT PRIMARY KEY,
    email VARCHAR(255) NOT NULL,
    username VARCHAR(100) NOT NULL,
);

-- Tabella dei ristoranti
IF NOT EXISTS CREATE TABLE ristorante (
    id_utente INT PRIMARY KEY,
    nome VARCHAR(100) NOT NULL,
    orario JSON,
    descrizione VARCHAR(511),
    indirizzo VARCHAR(255) NOT NULL,
    telefono VARCHAR(10),
    website VARCHAR(255),
    costo PRIZE_ENUM,
    sedie_per_bambini BOOLEAN,
	adatto_ai_disabili BOOLEAN,
    FOREIGN KEY (id_utente) REFERENCES utente(id)
);

-- Tabella delle prenotazioni
IF NOT EXISTS CREATE TABLE prenotazione (
    id INT AUTO_INCREMENT PRIMARY KEY,
    data_e_ora DATETIME NOT NULL,
    numero_persone INT NOT NULL,
    stato STATO_ORDINE NOT NULL,
    id_utente INT NOT NULL,
    id_ristorante INT NOT NULL,
    FOREIGN KEY (id_utente) REFERENCES utente(id),
    FOREIGN KEY (id_ristorante) REFERENCES ristorante(id_ristorante)
);

-- Tabella dei tag
IF NOT EXISTS CREATE TABLE tag (
    id INT AUTO_INCREMENT PRIMARY KEY,
    nome VARCHAR(50) NOT NULL,
    descrizione VARCHAR(255)
);

-- Tabella di collegamento tra tag e ristoranti
IF NOT EXISTS CREATE TABLE cucina (
    id_tag INT NOT NULL,
    id_ristorante INT NOT NULL,
    FOREIGN KEY (id_tag) REFERENCES tag(id),
    FOREIGN KEY (id_ristorante) REFERENCES ristorante(id)
);

-- Tabella delle ordinazioni
IF NOT EXISTS CREATE TABLE ordinazione (
    id_utente INT NOT NULL,
    id_prenotazione INT NOT NULL,
    id_piatto INT NOT NULL,
    quantita INT NOT NULL,
	PRIMARY KEY (id_utente, id_prenotazione, id_piatto),
    FOREIGN KEY (id_utente) REFERENCES utente(id),
    FOREIGN KEY (id_prenotazione) REFERENCES prenotazione(id),
    FOREIGN KEY (id_piatto) REFERENCES piatto(id)
);

-- Tabella dei menu dei ristoranti
IF NOT EXISTS CREATE TABLE menu (
    id_ristorante INT NOT NULL,
    id_piatto INT NOT NULL,
    prezzo DECIMAL(10, 2) NOT NULL,
	PRIMARY KEY (id_ristorante, id_piatto),
    FOREIGN KEY (id_ristorante) REFERENCES ristorante(id),
    FOREIGN KEY (id_piatto) REFERENCES piatto(id)
);

-- Tabella dei piatti
IF NOT EXISTS CREATE TABLE piatto (
    id INT AUTO_INCREMENT PRIMARY KEY,
    nome VARCHAR(100) NOT NULL,
    descrizione VARCHAR(511),
    foto JSON
);

-- Tabella di collegamento tra piatti e ingredienti
IF NOT EXISTS CREATE TABLE composizione (
    id_piatto INT NOT NULL,
    id_ingrediente INT NOT NULL,
    FOREIGN KEY (id_piatto) REFERENCES piatto(id),
    FOREIGN KEY (id_ingrediente) REFERENCES ingrediente(id)
);

-- Tabella degli ingredienti
IF NOT EXISTS CREATE TABLE ingrediente (
    id INT AUTO_INCREMENT PRIMARY KEY,
    nome VARCHAR(100) NOT NULL,
    descrizione VARCHAR(255),
    foto JSON
);

-- Tabella degli allergeni
IF NOT EXISTS CREATE TABLE allergene (
    id_ingrediente INT NOT NULL,
    allergene ALLERGENE NOT NULL,
    foto JSON,
	PRIMARY KEY (id_ingrediente, allergene),
    FOREIGN KEY (id_ingrediente) REFERENCES ingrediente(id)
);

-- Tabella delle allergie degli utenti
IF NOT EXISTS CREATE TABLE allergia (
    id_utente INT NOT NULL,
    allergene ALLERGENE NOT NULL,
    FOREIGN KEY (id_utente) REFERENCES utente(id),
);
