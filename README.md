# Kotik
Pack encryption utility for Minecraft Bedrock. Used for protecting resource packs on servers via encryption

## How to encrypt
First, create a folder where the files that you will encrypt will be located. Make sure that there is a manifest.json file in this folder
Use command `random_key` to generate random valid key for your encryption.
Next, use command `encrypt <your_key> <path_to_folder>` to encrypt your files.

## How to decrypt
For decryption you need to have a key that you used to encrypt this pack. Next, find the folder with encrypted content. It must contain the contents.json file.
Use command `decrypt <your_key> <path_to_folder>` to decrypt files.

**Warning:** This tool is not intended to and cannot break Marketplace DRMs. Intended only for protecting resource pack on the servers.
