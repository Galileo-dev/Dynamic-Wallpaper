import urllib.request
import json
import sys
import ctypes
import time
import os
from os import listdir
from os.path import isfile, join
from os import walk
from os import walk

from datetime import datetime

if (internet_on()):
	mypath = "./stored_backgrounds"
else: 
	mypath = "./stored_backgrounds/sun"


def wallpaper(mypath):
	imageFilename = getImageNames(mypath)
	try:
		setImageAsBackground(GetImage(imageFilename, ImageNum()))
	except:
		pass


		
	
def ImageNum():
	if (internet_on()):
		return 1
	else:
		return now() 
def now():
	now = datetime.now()
	current_time = now.strftime("%H")
	return current_time


def internet_on():
	try:
		urllib.request.urlopen('http://216.58.192.142', timeout=1)
		return True
	except urllib.request.URLError as err:
		return False

def GetImage(imageFilename, num):
	imageFilename.sort(key=len)
	i = imageFilename.index(str(num) + ".jpeg")
	return imageFilename[i]

def getImageNames(mypath):
	f = []
	for (dirpath, dirnames, filenames) in walk(mypath):
		
		f.extend(filenames)
		break
	return f

def setImageAsBackground(imageFilename):
	try:
		print("Attempting to print")
		ctypes.windll.user32.SystemParametersInfoW(20, 0, getFullPathOfImage(imageFilename) , 0)
		return imageFilename
	except:
		print("Changing Wallpaper Failed")
		return "None"
	

def getFullPathOfImage(imageFilename):
	try:
		return os.path.dirname(os.path.realpath("stored_backgrounds/" + imageFilename)) + "\\" + imageFilename
	except:
		return "None"
	


wallpaper(mypath)