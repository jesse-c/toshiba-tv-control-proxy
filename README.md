# Toshiba TV control proxy

This Toshiba TV (50VL5A63DB) is a rebranded Vestrel TV. There's also similarities to Telefunken TVs.

DIAL is available and there's the `SmartCenter` application, which allows sending key codes that may to remote control keys.

I'm using a Homebridge HTTP switch to turn it on and off, and to know the status—through Homekit (Siri). The issue with turning it on though is that sometimes the TV is in a state where it responds to Wake-on-LAN but not to DIAL requests. This control proxy sends _both_ when turning the TV on.
