
import pkgutil
import pkg_resources

try:
    import psycopg2
    print("psycopg2 = instalado")
except pkg_resources.DistributionNotFound:
    print("psycopg2 = NO instalado")
try:
    import sys
    print("sys = instalado")
except pkg_resources.DistributionNotFound:
    print("sys = NO instalado")
try:
    from  time import gmtime, strftime
    print("time = instalado")
except pkg_resources.DistributionNotFound:
    print("time = NO instalado")
try:
    from datetime import datetime
    print("datetime = instalado")
except pkg_resources.DistributionNotFound:
    print("datetime = NO instalado")
try:
    import os
    print("os = instalado")
except pkg_resources.DistributionNotFound:
    print("os = NO instalado")
try:
    import traceback
    print("traceback = instalado")
except pkg_resources.DistributionNotFound:
    print("traceback = NO instalado")
try:
    import ConfigParser
    print("ConfigParser = instalado")
except pkg_resources.DistributionNotFound:
    print("ConfigParser = NO instalado")
try:
    import ConfigParser
    print("ConfigParser = instalado")
except pkg_resources.DistributionNotFound:
    print("ConfigParser = NO instalado")
try:
    import glob
    print("glob = instalado")
except pkg_resources.DistributionNotFound:
    print("glob = NO instalado")
try:
    import base64
    print("base64 = instalado")
except pkg_resources.DistributionNotFound:
    print("base64 = NO instalado")
try:
    import socket
    print("socket = instalado")
except pkg_resources.DistributionNotFound:
    print("socket = NO instalado")
try:
    import pyodbc
    print("pyodbc = instalado")
except pkg_resources.DistributionNotFound:
    print("pyodbc = NO instalado")
try:
    import shutil
    print("shutil = instalado")
except pkg_resources.DistributionNotFound:
    print("shutil = NO instalado")
try:
    import shutil
    print("lxml = instalado")
except pkg_resources.DistributionNotFound:
    print("lxml = NO instalado")


"""
#version = pkg_resources.get_distribution("psycopg2").version
#print("psycopg2 instalado : " + version)


#try:
#    import psycopg2
#    import sys
#    import time
#    from datetime import datetime
#    from os import listdir
#    from lxml import etree
#    import os.path as path
#    import traceback
#    from ConfigParser import ConfigParser
#    import os, glob
#    import base64
#    import socket
#    import pyodbc
#    import shutil
#    print("TODOS LOS PAQUETES IMPORTADOS")
#except pkg_resources.DistributionNotFound:
#    print("Algo no se instalo")
"""