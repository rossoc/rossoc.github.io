# Crash Course su Docker

## Che cos'è Docker?

Docker è una piattaforma open-source che permette di creare, distribuire e
gestire applicazioni in contenitori. I contenitori Docker sono ambienti 
isolati che includono tutto il necessario per eseguire un'applicazione, come 
librerie, dipendenze e codice, garantendo la portabilità e la facilità di 
distribuzione su diversi ambienti.

## Overview dei punti successivi

    - Che cos'è Docker
    - Utilità di Docker
    - Costruzione di un Dockerfile generale
    - Dockerfile per una repo TypeScript
    - Docker Compose
    - Utilità di Docker Compose
    - Docker Compose per un'applicazione con Postgres, Node (Nest.js) e Angular

## Spiegazione di Che Cosa Sia Docker

Docker è una piattaforma che utilizza container per isolare e distribuire 
applicazioni insieme a tutte le loro dipendenze. I container consentono di 
eseguire applicazioni in un ambiente isolato, garantendo la portabilità e la 
consistenza tra ambienti diversi. Docker è come un sistema operativo e ciascun container è come un utente del sistema operativo. Questo vuol dire che i container singolarmente sono abbastanza leggeri e contengono le informazioni minimali per poter essere eseguiti e gestiti da Docker.
In realtà Docker non fornisce l'interfaccia grafica (ci sono eccezioni), per cui risulta essere molto più leggero di un vero sistema operativo. Si tratta più che altro di una macchina virtuale, molto più leggera e minimale.

## Spiegazione del Perché Docker Sia Utile

Docker semplifica lo sviluppo, la distribuzione e la gestione delle 
applicazioni. Consente di eliminare le differenze tra ambienti di sviluppo, 
test e produzione, garantendo la coerenza tra i vari passaggi del ciclo di 
vita dell'applicazione. Proprio perché il codice è eseguito all'interno di una macchina virtuale. Un container di docker corrisponde ad un'immagine di una macchina virtuale, in cui tutti i pacchetti e il sistema operativo alla base è messo a "fattor comune" tra tutti i container Docker ed è contenuto da Docker stesso. Infatti per eseguire un container Docker è necessario avviare Docker.

## Dockerfile Generico

Il Dockerfile è un file di testo che contiene una serie di istruzioni per la 
creazione di un'immagine Docker. Di seguito un esempio generico di Dockerfile:

```Dockerfile
# Indica l'immagine base da cui partire
FROM ubuntu:latest

# Copia i file del codice sorgente nell'immagine
COPY . /app

# Esegui comandi per configurare l'ambiente
RUN apt-get update && apt-get install -y python3

# other commands
# RUN ...

# Definisci il comando di avvio dell'applicazione
CMD ["python3", "/app/main.py"]
```

Posizionamento all'interno del progetto: il Dockerfile dovrebbe essere situato 
nella radice del progetto o nella cartella principale dell'applicazione.
>**NB** Per eseguire qualunque comando di docker bisogna avere Docker aperto in background.

1. Costruire l'image di un progetto software a partire dal Dockerfile in esso contenuto:

	```bash
	docker build -t nome_image /path/to/project/home
	```

1. Crea un'istanza dell'immagine, ovvero un container in esecuzione:

	```bash
	docker run -d -p 3000:3000 --name containter_name nome_image
	```

**NB** quando viene eseguito ``docker run`` il nome dell'immagine deve essere indicato come ultimo argomento.
- la flag ``-d`` indica a docker di eseguire il container in ``detached mode``, ovvero gli eventuali output prodotti dal container sul terminale non sono mostrati. Fondamentalmente, l'esecuzione del container avviene in background.
- la flag ``-p local_port:docker_container_port`` mappa una porta locale con una porta del container di docker.
- la flag ``-name container_name`` assegna un nome al container.

Di seguito una lista di comandi per gestire le image e i container di docker.
- ``docker image ls``: elenco delle immagini locali.
- ``docker image rm``: cancella un immagine (come ``rm``), notabene, per cancellare un'immagine non ci devono essere container da essa inizializzati
- ``docker ps`` oppure ``docker container ls``: elenco dei container.
- ``docker stop container_name``: mette in pausa il container in questione.
- ``docker start container_name``: riavvia il container in questione.
- ``docker rm container_name``: cancella il container in questione, notabene, per rimuovere un container questo deve prima essere in pausa.
- ``docker exec -it container_name /bin/bash``: entra dentro il container,
  l'interfaccia è il terminale ovviamente. Vi chiederete come mai sto includendo
  questo comando. Ebbene è molto utile per trovare eventuali bug.

## Dockerfile per Repo TypeScript

Se si desidera costruire un Dockerfile per un progetto TypeScript, è necessario 
considerare l'utilizzo del runtime di Node.js e la gestione delle dipendenze. 
Ecco un esempio di Dockerfile per un'applicazione TypeScript:

```Dockerfile
# Usa un'immagine di Node.js come base
FROM node:latest

# Imposta la directory di lavoro all'interno del container
# La direcotry dalla quale sono eseguiti i comandi
WORKDIR /usr/src/app

# Copia il file package.json e package-lock.json
COPY package*.json ./

# Installa le dipendenze
RUN npm install

# Copia il resto del codice sorgente nell'immagine
COPY . .

# Esponi la porta su cui l'applicazione ascolta
EXPOSE 3000

# Comando di avvio dell'applicazione
CMD ["npm", "start"]
```

Posizionamento nel progetto: Questo Dockerfile dovrebbe essere collocato nella 
radice del progetto TypeScript, dove si trova il file package.json.

## **Spiegazione di Che Cosa Sia Docker Compose**

Docker Compose è uno strumento per definire e gestire applicazioni multi-
container. Consente di definire le configurazioni di più container all'interno 
di un singolo file, semplificando la gestione e l'orchestrazione di più 
servizi Docker. Quindi, un Dockerfile spiega come generare un'immagine, una sorta di ISO. 
Un'image può essere eseguita e dunque sarà creato un container da essa inizializzato. Un container può essere messo in pausa, riavviato o eliminato.
Nel momento in cui si hanno più immagini, per gestirle tutte e per creare i container a partire da esse si utilizza docker compose.

## **Scopo di Docker Compose**

Docker Compose semplifica lo sviluppo e la gestione di applicazioni che 
richiedono più servizi, come ad esempio un database, un server backend e un'
interfaccia frontend. Consente di avviare e interconnettere più container con 
una singola istruzione.

## Docker Compose

Il Docker Compose è utilizzato per definire e gestire servizi multi-container. 
Di seguito un esempio di Docker Compose per un'applicazione con Postgres come 
database, Node.js (Nest.js) per il backend e Angular per il frontend:

```yaml
version: '3'
services:
  postgres:
    image: postgres:latest
    environment:
      - POSTGRES_USER=user
      - POSTGRES_PASSWORD=postgres
      - POSTGRES_DB=testDB
	network:
	  - network_name

  backend:
    build: ./backend
    ports:
      - 3000:3000
    depends_on:
      - postgres
	network:
	  - network_name

  frontend:
    build: ./frontend
    ports:
      - 80:80
    depends_on:
      - backend
	  
  service_name:
	build: ./project/directory/with/Dockerfile
	ports:
	  - local_port:container_port
	  - local_port:container_port
   
     # quali servizi avviare prima di questo
     depends_on:
      - another_service_in_the_docker_compose

# Il nome di un network fittizio simulato da docker-compose
network:
  network_name:
	  driver: bridge
```

Posizionamento all'interno del progetto: Il file docker-compose.yml dovrebbe 
essere collocato nella radice del progetto. Le cartelle backend e frontend 
devono contenere i rispettivi Dockerfile e il codice sorgente delle applicazioni 
corrispondenti.
Un servizio indica a docker-compose in quale modo andare a costruire le immagini ed in quale modo eseguire le immagini. Con docker-compose è possibile definire un network interno, una rete simulata per permettere ai container di comunicare.
Per comunicare nella rete il nome dell'host coincide con il nome del servizio. Per esempio, una connessione https diventa:

```ts
import axios, { AxiosResponse, AxiosError } from 'axios';

interface UserData {
  username: string;
  email: string;
  // ... other fields
}

const userData: UserData = {
  username: 'example_user',
  email: 'user@example.com',
  // ... other data
};

axios.post('https://service_name:port/api', userData)
  .then((response: AxiosResponse) => {
    console.log('POST request successful!');
    console.log('Response data:', response.data);
    // Handle response data here
  })
  .catch((error: AxiosError) => {
    console.error('Error making POST request:', error.message);
    // Handle errors here
  });
```

- Per eseguire i container con docker composer si usa il comando:
```bash
docker compose up
```

- Per bloccare l'esecuzione di docker composer si utilizza la combinazione:
```bash
<C-c>
```
Ovvero ```Control + c```.
