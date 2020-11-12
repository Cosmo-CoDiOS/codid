#!/usr/bin/env python3
import argparse
import sys
import time
from datetime import datetime

from gi.repository import GLib
import DBusServer
import LEDManager
import CodiFunctions as cf
import SerialPortManager
import codi_mtk_generated_functions as mtkCmd
from codi_generated_parser import *
import signal

import CodiStatus
import EventListener
import Addressbook

def signalHandler(_signo, _stack_frame):
    # mtkCmd.SetMouse(0, 1)
    mtkCmd.SetCoDiStatus(3, 3, 3)
    sys.exit(0)

signal.signal(signal.SIGINT, signalHandler)
signal.signal(signal.SIGTERM, signalHandler)
signal.signal(signal.SIGABRT, signalHandler)
signal.signal(signal.SIGHUP, signalHandler)



CodiStatus.init()

def initCodi():
    Addressbook.refreshContacts()
    mtkCmd.SetCoDiStatus(1, 7, 1)
    mtkCmd.SetMouse(1, 1)
    cf.GetDateTime()
    LEDManager.ledsOff()
    mtkCmd.DoNotDisturbStatusInfo(0)
    mtkCmd.BTStatusInfo(0)
    mtkCmd.WiFiStatusInfo(1, 100)
    mtkCmd.ModemSignalInfo(1, 0, 0)
    mtkCmd.MTKDataChangeAlert(1, 0)
    mtkCmd.MTKDataChangeAlert(0, 0)
    cf.SetCallOutput(0)

SerialPortManager.init()

print('Codi Linux Server')
if args['command']:
    if args['command'] == 'dbus':
        try:
            DBusServer.init(False)
            eval(args['cmd'])
        except Exception as e:
            print(e)
        SerialPortManager.stop()
        exit(0)
    else:
        firstArg = True
        cmd = 'mtkCmd.' + args['command'] + '('
        for i in args:
            if i != 'command':
                if firstArg:
                    firstArg = False
                else:
                    cmd += ', '
                cmd += str(args[i])
        cmd += ')'
        eval(cmd)
        SerialPortManager.stop()
        exit(0)

EventListener.init()
initCodi()

DBusServer.init()

