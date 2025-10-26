# SSH: generare chiavi e (opzionale) CA offline

Questo documento spiega come lavorare offline per generare chiavi SSH, creare una CA locale e firmare chiavi utente. I file script sono in `./scripts`.

Attenzione: NON committare mai chiavi private o la private key della CA nel repository.

1) Generare una chiave utente (script fish)

File: `scripts/generate_ssh_key.fish`

Esempio d'uso (fish):

```fish
./scripts/generate_ssh_key.fish                # crea ~/.ssh/id_ed25519 e la carica nell'agent
./scripts/generate_ssh_key.fish ~/.ssh/mykey "mio@comment" --nopass
```

Lo script imposta i permessi corretti su `~/.ssh`, avvia `ssh-agent` e aggiunge la chiave.

2) Inizializzare una CA locale e firmare chiavi (offline)

File: `scripts/create_ssh_ca.sh`

Inizializzare la CA (esempio):

```bash
./scripts/create_ssh_ca.sh init ~/.local/share/ssh_ca
```

Questo crea:
- `~/.local/share/ssh_ca/ssh_ca` (PRIVATE — tienila offline e sicura)
- `~/.local/share/ssh_ca/ssh_ca.pub` (PUBLIC — distribuisci ai server)

Firmare una chiave utente (pubkey):

```bash
./scripts/create_ssh_ca.sh sign ~/.local/share/ssh_ca ~/.ssh/id_ed25519.pub "mio-username" +52w
```

Questo genera un file `id_ed25519-cert.pub` nella stessa cartella della public key.

3) Configurare i server per fidarsi della CA (richiede accesso admin)

Sul server copy `ssh_ca.pub` in `/etc/ssh/ca.pub` (o altra posizione leggibile) e modifica `/etc/ssh/sshd_config` aggiungendo:

```
TrustedUserCAKeys /etc/ssh/ca.pub
```

Poi riavvia `sshd`:

```bash
sudo systemctl restart sshd
```

Ora i client che presentano un certificato firmato dalla CA (file `id_ed25519-cert.pub` accanto alla private key) potranno autenticarsi senza che ogni server abbia entries in `authorized_keys`.

4) Aggiungere la chiave ai servizi (GitHub/GitLab) — offline -> web UI

- Visualizza la chiave pubblica con:

```fish
cat ~/.ssh/id_ed25519.pub
```

- Copia il contenuto e incollalo nelle impostazioni SSH del servizio (GitHub/GitLab).

5) Note di sicurezza e best practice
- Proteggi sempre la private key con una passphrase se possibile.
- Non inserire mai chiavi private nel codice o nel repository.
- Conserva la private key della CA in un luogo sicuro (preferibilmente offline/hsm) e crea procedure di backup e rotazione.

Se vuoi, posso:
- Aggiungere un piccolo Makefile o comando `scripts/install-ssh-tools` per rendere eseguibili gli script
- Preparare esempi `sshd_config` per sistemi Debian/RedHat
- Creare una routine di revoca per certificati (es. distribuire file CRL o scambio della CA.pub)

Ho aggiunto uno script `scripts/install-ssh-tools` e un `Makefile` con il target `ssh-tools`.

Esempio rapido per iniziare (da repository root, fish):

```fish
chmod +x scripts/*                       # (se non vuoi usare make)
make ssh-tools                           # esegue scripts/install-ssh-tools
./scripts/generate_ssh_key.fish          # genera una chiave e la carica nell'agent
./scripts/create_ssh_ca.sh init ~/.local/share/ssh_ca
```

Questi comandi sono pensati per un flusso offline: generi chiavi, inizializzi la CA locale e firmi chiavi senza bisogno di accesso ad Internet.

