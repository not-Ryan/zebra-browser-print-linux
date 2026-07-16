# Zebra Browser Print Linux

A cru implementation for
[Zebra Browser Print](https://www.zebra.com/us/en/support-downloads/software/printer-software/browser-print.html).

## Installations

Clone this repository and run `make install`:

```bash
git clone https://github.com/not-Ryan/zebra-browser-print-linux.git
make install
```

See [`install` recipe in makefile](./makefile) for the manual installation
instructions.

### Why!?

Because Zebra doesn't easily provide the Linux drivers for Zebra Browser Print.
You require to request them from the Zebra overlords
[here](https://support.zebra.com/article/000022132?redirect=false).

And I'm definitely not installing a Windows vmware just for printing.

### How...

By installing the [CUPS Driver](https://github.com/mvnural/zebra-cups-driver)
you can send raw ZPL commands through CUPS for example:

```bash
printf "^XA\n^PW400\n^LL400\n^FO10,10^GB80,80,3^FS\n^FO10,100^A0N,25,25^FD10mm@203dpi^FS\n^FO10,150^GB118,118,3^FS\n^FO10,280^A0N,25,25^FD10mm@300dpi^FS\n^XZ" \
  | lp -d <your-printer-name> -o raw -
```

I got this command and idea from another
[repository](https://github.com/boberbyte/zebra-gk420d-linux)

Then we spin up a server and just listen for commands.
