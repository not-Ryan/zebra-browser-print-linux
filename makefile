install:
	cargo build --release
	sudo mv ./target/release/zebra-browser-print /usr/sbin/zebra-browser-print
	sudo cp ./zebra-browser-print.service /etc/systemd/system/zebra-browser-print.service
	sudo systemctl daemon-reload
	sudo systemctl enable zebra-browser-print
	sudo systemctl start zebra-browser-print

uninstall:
	sudo systemctl stop zebra-browser-print
	sudo systemctl disable zebra-browser-print
	sudo rm /usr/sbin/zebra-browser-print
	sudo rm /etc/systemd/system/zebra-browser-print.service