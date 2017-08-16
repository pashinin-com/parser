from __future__ import print_function

__title__ = 'rparser'
__version__ = '0.1.22'
# __build__ = 0x021203
__author__ = 'Sergey Pashinin'
__license__ = 'GPL 3.0'
__copyright__ = 'Copyright 2017 Sergey Pashinin'


try:
    from .librparser import *  # noqa
except Exception as e:
    print("Error importing rparser: ", str(e))
