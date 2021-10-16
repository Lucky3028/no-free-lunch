#!/bin/bash
# systemctlでバイナリを起動・管理するsh。systemctlで呼ぶ側。
chmod +x restart-bot.sh
# sudoなしのユーザ権限でsystemctlを動かす。
mv -f run-no-free-lunch.service ~/.config/systemd/user/
systemctl enable --user run-no-free-lunch
systemctl daemon-reload --user
systemctl restart --user run-no-free-lunch