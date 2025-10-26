#!/usr/bin/env fish
# Simple helper to generate an ed25519 key, set permissions and add to ssh-agent
# Usage:
#   generate_ssh_key.fish [path] [comment] [--nopass]

# Resolve outpath and comment from args
if test (count $argv) -ge 1
    set -l outpath $argv[1]
else
    set -l outpath ~/.ssh/id_ed25519
end

if test (count $argv) -ge 2
    set -l comment $argv[2]
else
    set -l comment (whoami)"@"(hostname)
end

set -l nopass false
for a in $argv
    if test "$a" = "--nopass"
        set nopass true
    end
end

if not test -d ~/.ssh
    mkdir -p ~/.ssh
    chmod 700 ~/.ssh
end

if test -f $outpath
    if test $nopass = true
        # non-interactive mode: remove existing files
        rm -f $outpath $outpath.pub
    else
        echo "File $outpath esiste già. Sovrascrivere? (y/N)"
        read -l answer
        if test "$answer" != "y" -a "$answer" != "Y"
            echo "Annullato. Scegli un file diverso o rimuovi quello esistente."; exit 1
        end
        rm -f $outpath $outpath.pub
    end
end

if test $nopass = true
    ssh-keygen -t ed25519 -f $outpath -C "$comment" -N ""
else
    ssh-keygen -t ed25519 -f $outpath -C "$comment"
end

chmod 600 $outpath
chmod 644 $outpath.pub

echo "Avvio ssh-agent e aggiunta della chiave..."
eval (ssh-agent -c)
ssh-add $outpath

echo "Fatto. Chiave pubblica disponibile in: $outpath.pub"
echo "Per aggiungerla a GitHub/GitLab copia il contenuto di quel file e incollalo nell'interfaccia web." 

exit 0
#!/usr/bin/env fish
# Simple helper to generate an ed25519 key, set permissions and add to ssh-agent
# Usage:
#   generate_ssh_key.fish [path] [comment] [--nopass]

# Resolve outpath and comment from args
if test (count $argv) -ge 1
    set -l outpath $argv[1]
else
    set -l outpath ~/.ssh/id_ed25519
end

if test (count $argv) -ge 2
    set -l comment $argv[2]
else
    set -l comment (whoami)"@"(hostname)
end

set -l nopass false
for a in $argv
    if test "$a" = "--nopass"
        set nopass true
    end
end

if not test -d ~/.ssh
    mkdir -p ~/.ssh
    chmod 700 ~/.ssh
end

if test -f $outpath
    if test $nopass = true
        # non-interactive mode: remove existing files
        rm -f $outpath $outpath.pub
    else
        echo "File $outpath esiste già. Sovrascrivere? (y/N)"
        read -l answer
        if test "$answer" != "y" -a "$answer" != "Y"
            echo "Annullato. Scegli un file diverso o rimuovi quello esistente."; exit 1
        end
        rm -f $outpath $outpath.pub
    end
end

if test $nopass = true
    ssh-keygen -t ed25519 -f $outpath -C "$comment" -N ""
else
    ssh-keygen -t ed25519 -f $outpath -C "$comment"
end

chmod 600 $outpath
chmod 644 $outpath.pub

echo "Avvio ssh-agent e aggiunta della chiave..."
eval (ssh-agent -c)
ssh-add $outpath

echo "Fatto. Chiave pubblica disponibile in: $outpath.pub"
echo "Per aggiungerla a GitHub/GitLab copia il contenuto di quel file e incollalo nell'interfaccia web." 

exit 0
