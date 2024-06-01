# Pifijs the Discord bot

## Running
```
nix build --option sandbox false

# Run server
export PIFIJS_RENDERER="$(pwd)/result/pifijs_plotter"
export DISCORD_TOKEN="..."
nix develop -c result/pifijs_bot

# Run standalone plotter
nix develop -c "result/pifijs_plotter '0,1>0,-1 -1,0>1,0'"
nix develop -c "result/pifijs_plotter '0,1>0,-1 -1,0>1,0' /tmp/test.png"
```

## Usage
- Invite the bot to your Discord server  
- Send a command to the bot e.g. `!plot 0,1>0,-1 -1,0>1,0`  
