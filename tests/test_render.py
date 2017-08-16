# import os

from rparser import article_render as render

def test_normal():
    src = 'asd'
    html, info = render(src)
    assert '<p>asd</p>' == html


    src = r'\youtube{code}'
    html, info = render(src)
    # assert 'yt' == html

    src = r'\file{sha1} \file{sha2}'
    html, info = render(src, files={'sha2': {1: 2}})
    # assert 'yt' == html
    # assert html == ''

    # \file{a9a7d18e7afe12c7e6ebfbafbb997793c1225250, w=100}
