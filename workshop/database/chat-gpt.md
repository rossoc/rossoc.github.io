#### Cliente
- email VARCHAR(255) NOT NULL
- nome_utente VARCHAR(100) NOT NULL
- prenotazione (0,N)
- ordinazione (0,N)
- allergene (0,N)

#### Ristoratore
- email VARCHAR(255) NOT NULL
- nome_utente VARCHAR(100) NOT NULL
- ristorante (0,1)

#### Ristorante
- nome VARCHAR(100) NOT NULL
- orario VARCHAR(50) NOT NULL
- descrizione TEXT
- indirizzo VARCHAR(255) NOT NULL
- recapiti VARCHAR(255)
- website VARCHAR(255)
- costo DECIMAL(10, 2)
- sedie_per_bambini BOOLEAN
- adatto_a_persona_con_ridotta_mobilità BOOLEAN
- ristoratore (1,1)
- tag (0,N)

#### Prenotazione
- data DATE NOT NULL
- ora TIME NOT NULL
- numero_persone INT NOT NULL
- stato VARCHAR(50) NOT NULL
- ristorante (1,1)
- utente (1,1)
- ordinazione (0,1)

#### Ordinazione
- costo_parziale DECIMAL(10, 2) NOT NULL
- utente (1,1)
- prenotazione (1,1)
- piatto (0,N)

#### Piatto
- foto VARCHAR(255)
- nome VARCHAR(100) NOT NULL
- descrizione TEXT
- prezzo DECIMAL(10, 2) NOT NULL
- ingrediente (0,N)
- ristorante (1,1)

#### Ingrediente
- foto VARCHAR(255)
- nome VARCHAR(100) NOT NULL
- descrizione TEXT
- prezzo_quantità VARCHAR(100)
- piatto (1,1)
- allergene (0,N)

#### Allergene
- nome VARCHAR(100) NOT NULL
- descrizione TEXT
- foto VARCHAR(255)
- piatto (0,N)
- utente (0,N)
- ingrediente (1,1)

#### Tag
- nome VARCHAR(100) NOT NULL
- descrizione TEXT
- ristorante (0,N)

Segue la tabella delle relazioni:

| Relazione | Entità coinvolte | Descrizione | Attributi |
|-----------|------------------|-------------|-----------|
| Cliente_Prenotazione | Cliente (1,1), Prenotazione (0,N) | Relazione tra Cliente e le sue Prenotazioni | - |
| Cliente_Ordinazione | Cliente (1,1), Ordinazione (0,N) | Relazione tra Cliente e le sue Ordinazioni | - |
| Cliente_Allergene | Cliente (0,N), Allergene (0,N) | Relazione tra Cliente e gli Allergeni | - |
| Ristoratore_Ristorante | Ristoratore (1,1), Ristorante (0,1) | Relazione tra Ristoratore e il suo Ristorante | - |
| Ristorante_Tag | Ristorante (0,N), Tag (0,N) | Relazione tra Ristorante e i Tag | - |
| Prenotazione_Ordinazione | Prenotazione (0,1), Ordinazione (1,1) | Relazione tra Prenotazione e Ordinazione | - |
| Ordinazione_Piatto | Ordinazione (1,1), Piatto (0,N) | Relazione tra Ordinazione e Piatti | - |
| Piatto_Ingrediente | Piatto (1,1), Ingrediente (0,N) | Relazione tra Piatti e Ingredienti | - |
| Ingrediente_Allergene | Ingrediente (1,1), Allergene (0,N) | Relazione tra Ingredienti e Allergeni | - |
