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

mypath = "./stored_backgrounds"


def wallpaper(mypath):
	imageFilename = getImageNames(mypath)
	try:
		imageUsed = setImageAsBackground(GetImage(imageFilename, now()))
	except:
		print("Failed to get image")
		exit()
	return "Mojave" + "_{0}".format(imageUsed)


def now():
	now = datetime.now()
	current_time = now.strftime("%H")
	return current_time


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
		ctypes.windll.user32.SystemParametersInfoW(20, 0, getFullPathOfImage(imageFilename) , 0)
		return imageFilename
	except:
		return "None"
	

def getFullPathOfImage(imageFilename):
	try:
		return os.path.dirname(os.path.realpath("stored_backgrounds/" + imageFilename)) + "\\" + imageFilename
	except:
		return "None"
	


image = wallpaper(mypath)