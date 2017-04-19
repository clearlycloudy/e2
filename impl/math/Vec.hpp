#ifndef VEC_HPP
#define VEC_HPP

#include <stdexcept>
#include <string>
class Vec {

 public:
                Vec(); // default 3D vector
                Vec(int dim); // set vector with certain dimension
                Vec(const Vec & v); // copy vector
                ~Vec();
  float *       _vec; // vector data
  int           _dim; // vector dimension
  void          SetDim(int); //resize dimension and preserve existing data if possible
  int           GetDim() const { return _dim; }
  Vec &         operator = (const Vec & v);
  Vec           operator + (const Vec & v) const;
  Vec           operator - (const Vec & v) const;
  Vec           operator / (const Vec & v) const;
  bool          IsEqual(const Vec & v, float error) const;
  inline float &    operator [] ( int i ){ return _vec[i]; };
  inline float      operator [] ( int i ) const{ return _vec[i]; };
  float         Dot(const Vec & v) const;
  Vec           Cross(const Vec & v) const;

  float         Magnitude() const;
  void          NormalizeCurrent(); //normalize current vec
  Vec           Normalize() const; //return a normalize vec

  void          SetFromArray(int dim, float array[] ); //copy from array
  void          SetFromArray(int dim, double * array ); //copy from array
  void          GetArray(int & dim, float * & array ) const; //copy to array with memory allocation
  bool          GetArray(float * dest, size_t dest_count, size_t & actual_count ) const; //copy without memory allocation to destination
  Vec           GetSubVector( int index_start, int index_end ) const;
  class Exception : public std::runtime_error {
  public:
    Exception(const std::string &msg): std::runtime_error(msg) { }
  };
  
static Vec      ScaleVec(float s, const Vec v); //s * v
static Vec      ScaleVecAdd(float s, const Vec v1, const Vec v2);//s * v1 + v2
};

#endif