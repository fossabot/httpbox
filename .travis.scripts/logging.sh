#!/usr/bin/env bash

RESET=$(echo -en '\033[0m')
RED=$(echo -en '\033[00;31m')
GREEN=$(echo -en '\033[00;32m')
YELLOW=$(echo -en '\033[00;33m')
BLUE=$(echo -en '\033[00;34m')
MAGENTA=$(echo -en '\033[00;35m')
PURPLE=$(echo -en '\033[00;35m')
CYAN=$(echo -en '\033[00;36m')
LIGHTGRAY=$(echo -en '\033[00;37m')
LRED=$(echo -en '\033[01;31m')
LGREEN=$(echo -en '\033[01;32m')
LYELLOW=$(echo -en '\033[01;33m')
LBLUE=$(echo -en '\033[01;34m')
LMAGENTA=$(echo -en '\033[01;35m')
LPURPLE=$(echo -en '\033[01;35m')
LCYAN=$(echo -en '\033[01;36m')
WHITE=$(echo -en '\033[01;37m')
ALLOW_TERM_COLORS=$(tput colors >/dev/null 2>&1; if [ 0 -eq $? ]; then echo "true"; else echo "false"; fi)

_timestamp () {
  date -u +"%Y-%m-%d %H:%M:%SZ"
}

log() {
  echo "[$(_timestamp) LOG]: $*"
}

warn() {
  [ "true" = ${ALLOW_TERM_COLORS} ] && echo "${YELLOW}[$(_timestamp) WRN]: $*${RESET}" || log "$*"
}

error() {
  [ "true" = ${ALLOW_TERM_COLORS} ] && echo "${RED}[$(_timestamp) ERR]: $*${RESET}" || log "$*"
}

info() {
  [ "true" = ${ALLOW_TERM_COLORS} ] && echo "${GREEN}[$(_timestamp) INF]: $*${RESET}" || log "$*"
}

debug() {
  [ "true" = ${ALLOW_TERM_COLORS} ] && echo "${MAGENTA}[$(_timestamp) DEB]: $*${RESET}" || log "$*"
}

emptyline() {
  echo ""
}

emptylog() {
  log ""
}
