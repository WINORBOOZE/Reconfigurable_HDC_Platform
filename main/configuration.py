"""
/**
 * @file configuration.py
 * @brief This file contains the implementation of the Configuration class: a class that manages the configuration of the project.
 * @author Marco Angioli and Saeid Jamili
 * @email marco.angioli@uniroma1.it and saeid.jamili@uniroma1.it
 * @note Author names are listed in alphabetical order.
 * @date Created on: 12th August 2023
 * @date Last updated on: 5th May 2024
 * @institution Sapienza University of Rome
 *
 * @section LICENSE
    Copyright 2024 Sapienza University of Rome

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
    Authors: Marco Angioli & Saeid Jamili

 * @section CHANGELOG
 * @version 1.0.2
 * @date May 2024
 */
"""
import json
import os
import subprocess
import subprocess
import os
import sys
import shutil
import subprocess
import time
import webbrowser
import signal
import datetime
from PyQt5.QtWidgets import QApplication
from PyQt5.QtWebEngineWidgets import QWebEngineView
from PyQt5.QtCore import QUrl, Qt
from PyQt5.QtGui import QIcon
from guiconfig import GUIConfig
from IPython.core.display import display, HTML
from utils import Utils


DEFAULT_CONFIG_PATH = "./usr_config/config.json"
CONFIG_GEN_PATH = "./main/config_gen.py"

class Configuration:
    def __init__(self, file_name=DEFAULT_CONFIG_PATH):
        self.config_data = self.read(file_name)

    def read(self, file_name=DEFAULT_CONFIG_PATH):
        """Reads the configuration file."""
        if not os.path.exists(file_name):
            raise FileNotFoundError(f"Config file {file_name} not found.")
        
        with open(file_name, 'r') as f:
            self.config_data = json.load(f)
            return self.config_data

    def print(self):
        """Displays the entire configuration."""
        print(json.dumps(self.config_data, indent=4))

    def get_value(self, parameter_name):
        """Returns the value of a given parameter name."""
        keys = parameter_name.split('.')
        data = self.config_data

        for key in keys:
            data = data.get(key)
            if data is None:
                raise KeyError(f"Parameter {parameter_name} not found in config.")

        if 'selected' in data:
            return data['selected']

        raise ValueError(f"Parameter {parameter_name} does not have a 'selected' field.")

    def get_options(self, parameter_name):
        """Returns the possible options for a given parameter name."""
        keys = parameter_name.split('.')
        data = self.config_data

        for key in keys:
            data = data.get(key)
            if data is None:
                raise KeyError(f"Parameter {parameter_name} not found in config.")

        if 'options' in data:
            return list(data['options'].keys())

        raise ValueError(f"Parameter {parameter_name} does not have an 'options' field.")

    def save(self, new_config=None, filename=DEFAULT_CONFIG_PATH):
        """Saves the configuration file."""
        if new_config:
            self.config_data = new_config

        with open(filename, 'w') as f:
            json.dump(self.config_data, f, indent=4)

    def compile(self):
        """Runs the config_gen.py script."""
        if not os.path.exists(CONFIG_GEN_PATH):
            raise FileNotFoundError(f"Config script {CONFIG_GEN_PATH} not found.")
        
        subprocess.run(["python", CONFIG_GEN_PATH])


    def print(self):
        html_output = "<div style='font-family: Arial, sans-serif; font-size: 14px;'>"
        html_output += Utils.display_dict(self.config_data)
        html_output += "</div>"

        display(HTML(html_output))



    def run_gui(self):
        GUIConfig.update()
        url = "http://localhost:3000/usr_config.html"
        webbrowser.open(url)
        GUIConfig.update()

