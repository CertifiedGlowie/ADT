Usage:

```ADT.exe --file PATHTOFILE --key HEXEDKEY```

or (if you on linux):

```./ADT --file PATHTOFILE --key HEXEDKEY```

Keys are accepted only in hexadecimal format.

Q & A:

Q: What programming language it's using?: 
A: It's originaly written in rust 1.74.1

Q: What is the purpose of this application? 
A: To allow secure file transfering and easy decryption. I did not add encryption modules because you only need to change one word in the source code (from decrypt_block in line 63 to encrypt_block) and because AVs might flag it as ransomware.

Q: Will it be updated?: 
A: Unfortunately, i do not plan on updating this. Hovewer that doesn't mean that there will never be an update.

Q: What about the license/copyright? 
A: Project is licensed under BSD clause 2 licenese, hovewer it's too simple to claim it to my name. I allow unconditional use and distribution of this software with or without copyright notice.
