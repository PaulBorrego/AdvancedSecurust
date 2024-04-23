Securust uses the orion library to encode files for individual users.
Using auad::SecretKey to seal and open files which encrypts files of any type.


The program will make users using an iced gui and will hold the users information and SecretKey in a file.
The program will then attempt to get a username and passsord from the system in and if one is accepted the
program will allow Encrypting and Decrypting into the User file.


Problems to be fixed and Additions to be made

1. The file that holds information is not itself encrypted, obvioulsy this is something we will fix but for now...
2. Make it so that a user is auto made when none exist
3. Add Safety features to make it that only the user can access there own user files
4. Convert the encrypt and decrypt into a file explorer gui
5. Implement a passwordhash
6. See if its possible to make it work over multiple computers.
