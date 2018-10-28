# Building C++ with MSVC
you have a weird system setup where your ucrtd (universal c runtime) is only available with windows 10 sdk 
but you have multiple sdks installed 8.0 and 8.1 and your visual studio 14 2015 expects to have the ucrtd

cmake will try to compile a test c++ project to see everything works and for this it needs to know which windows sdk to use
to override the default one use from the build/ folder
also remember that cmake caches stuff so always clear the cmake folder to start from fresh

```
cmake ../ -DCMAKE_SYSTEM_VERSION=10.0.14393.0
```