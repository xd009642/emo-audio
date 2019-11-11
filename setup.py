from setuptools import find_packages, setup
from setuptools_rust import RustExtension

setup_requires = ['setuptools-rust>=0.10.2']
install_requires = ['numpy']
test_requires = install_requires

setup(
    name='emo_audio',
    version='0.1.0',
    description='Emotech audio processing library',
    rust_extensions=[RustExtension('emo_audio.emo_audio',
        './Cargo.toml',
    )],
    install_requires=install_requires,
    setup_requires=setup_requires,
    test_requires=test_requires,
    packages=find_packages(),
    zip_safe=False,
)
