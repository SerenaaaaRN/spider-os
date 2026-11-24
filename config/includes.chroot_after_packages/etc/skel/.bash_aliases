# Prompt SpiderOS: [user@host dir] dengan palet hangat Spider-Sense
# 88 maroon, 180 sand, 223 cream — sama seperti render/ansi.rs
PS1='\[\e[38;5;88m\][\u\[\e[38;5;180m\]@\h \[\e[38;5;223m\]\W\[\e[38;5;88m\]]\[\e[0m\]\$ '

# Alias
alias ll='ls -lah --color=auto'
alias update='sudo apt update && sudo apt full-upgrade'
alias sense='spider-sense'

# Tampilkan neofetch sekali di terminal interaktif pertama
if [ -t 1 ] && command -v neofetch >/dev/null 2>&1 && [ -z "$SPIDEROS_GREETED" ]; then
    export SPIDEROS_GREETED=1
    neofetch
fi
