# rust_ml 🔥

**Whats included:**
 - The Instance and Loader modules ⚡:
   - Instances is a basic struct that holds a Vector of floats for the instance's points and an i32 for the lable.
   - Loader is used for reading and writing .inst files that contain Instances. Used to learn Rust's file IO.
 - The KNN module 🏠:
   - This module implements the k-Nearest Neighbors algorithm being trained/using the Instance module.
   - The main feature of this implmentation is the ablility to use multiple threads during classification. KNN is a lazy algortihm meaning it does no training until it is called to classify an Instance, due to this, each classification requires iterating over all training Instances. To multithread KNN, the training Instances are evenly divided among threads and iterated over allowing for much faster classifacation.
 - The Perceptron module 🧬:
   - Currently a basic implementation of a perceptron. 
   - Plans for the future include:
     - Multitheading the weight update portion of the implementation
     - Adding import and exporting weights
     - Abstracting the weight update function for future use in a neural network
