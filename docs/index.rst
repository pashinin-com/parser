.. rparser documentation master file, created by
   sphinx-quickstart on Wed Dec 14 20:46:13 2016.
   You can adapt this file completely to your liking, but it should at least
   contain the root `toctree` directive.

rparser - Python parsers (in Rust)
==================================

.. code-block:: bash

   pip install rparser

Headers
-------

As in Markdown:

.. code-block:: text

   # Header 1
   ## Header 2
   ### Header 3
   #### Header 4
   ##### Header 5
   ###### Header 6

Code
----

As in Github:

.. code-block:: text

   ```bash
   sudo apt-get install emacs
   ```

Inline code?

Lists
-----

Numbered:

.. code-block:: text

   #. Item 1
   #.#. Item 1.1
   #. Item 2

Will render as:

.. code-block:: html

    <ol>
      <li>Item 1</li>
      <li>Item 1.1</li>
      <li>Item 2</li>
    </ol>


Commands
--------

As in Latex:

.. code-block:: latex

   \command
   \command{content plus more content}


Links
-----

.. code-block:: latex

   \href{url}{text}

Complex blocks
--------------

.. code-block:: latex

   \Article{id}
   \Book{id}

Files
-----

.. code-block:: latex

   \file{sha1}

It depends how it will be rendered on what is this file: image, video,
sound, etc...

Image:

By default a single image is centered, size is original

An uploaded (known) file will render as:

.. code-block:: latex

   \file{sha1, w=100, h=100}

.. image:: images/Lenna.png



A missing file will render as:

Video
-----

.. code-block:: latex

   \youtube{video-code}


Tables
------

Like HTML:

.. code-block:: html

   <table>
     <tr>
       <td>
         content
       </td>
     </tr>
   </table>



Contents:

.. toctree::
   :maxdepth: 2

   rust-notes
   markdown


Indices and tables
==================

* :ref:`genindex`
* :ref:`modindex`
* :ref:`search`
