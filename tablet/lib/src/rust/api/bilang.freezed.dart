// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint, type=warning, deprecated_member_use, deprecated_member_use_from_same_package
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'bilang.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// GENERATED CODE - DO NOT MODIFY BY HAND
// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$AnswerDto {





@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is AnswerDto);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
    return 'AnswerDto()';
}


}

/// @nodoc
class $AnswerDtoCopyWith<$Res>  {
$AnswerDtoCopyWith(AnswerDto _, $Res Function(AnswerDto) __);
}


/// Adds pattern-matching-related methods to [AnswerDto].
extension AnswerDtoPatterns on AnswerDto {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( AnswerDto_Int value)?  int,TResult Function( AnswerDto_Frac value)?  frac,TResult Function( AnswerDto_Dec value)?  dec,TResult Function( AnswerDto_Remainder value)?  remainder,TResult Function( AnswerDto_Roots value)?  roots,TResult Function( AnswerDto_Pair value)?  pair,TResult Function( AnswerDto_Ineq value)?  ineq,required TResult orElse(),}){
final _that = this;
switch (_that) {
case AnswerDto_Int() when int != null:
return int(_that);case AnswerDto_Frac() when frac != null:
return frac(_that);case AnswerDto_Dec() when dec != null:
return dec(_that);case AnswerDto_Remainder() when remainder != null:
return remainder(_that);case AnswerDto_Roots() when roots != null:
return roots(_that);case AnswerDto_Pair() when pair != null:
return pair(_that);case AnswerDto_Ineq() when ineq != null:
return ineq(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( AnswerDto_Int value)  int,required TResult Function( AnswerDto_Frac value)  frac,required TResult Function( AnswerDto_Dec value)  dec,required TResult Function( AnswerDto_Remainder value)  remainder,required TResult Function( AnswerDto_Roots value)  roots,required TResult Function( AnswerDto_Pair value)  pair,required TResult Function( AnswerDto_Ineq value)  ineq,}){
final _that = this;
switch (_that) {
case AnswerDto_Int():
return int(_that);case AnswerDto_Frac():
return frac(_that);case AnswerDto_Dec():
return dec(_that);case AnswerDto_Remainder():
return remainder(_that);case AnswerDto_Roots():
return roots(_that);case AnswerDto_Pair():
return pair(_that);case AnswerDto_Ineq():
return ineq(_that);}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( AnswerDto_Int value)?  int,TResult? Function( AnswerDto_Frac value)?  frac,TResult? Function( AnswerDto_Dec value)?  dec,TResult? Function( AnswerDto_Remainder value)?  remainder,TResult? Function( AnswerDto_Roots value)?  roots,TResult? Function( AnswerDto_Pair value)?  pair,TResult? Function( AnswerDto_Ineq value)?  ineq,}){
final _that = this;
switch (_that) {
case AnswerDto_Int() when int != null:
return int(_that);case AnswerDto_Frac() when frac != null:
return frac(_that);case AnswerDto_Dec() when dec != null:
return dec(_that);case AnswerDto_Remainder() when remainder != null:
return remainder(_that);case AnswerDto_Roots() when roots != null:
return roots(_that);case AnswerDto_Pair() when pair != null:
return pair(_that);case AnswerDto_Ineq() when ineq != null:
return ineq(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( PlatformInt64 n)?  int,TResult Function( PlatformInt64 n,  PlatformInt64 d)?  frac,TResult Function( PlatformInt64 hundredths)?  dec,TResult Function( PlatformInt64 q,  PlatformInt64 r)?  remainder,TResult Function( PlatformInt64 a,  PlatformInt64 b)?  roots,TResult Function( PlatformInt64 x,  PlatformInt64 y)?  pair,TResult Function( String op,  PlatformInt64 n)?  ineq,required TResult orElse(),}) {final _that = this;
switch (_that) {
case AnswerDto_Int() when int != null:
return int(_that.n);case AnswerDto_Frac() when frac != null:
return frac(_that.n,_that.d);case AnswerDto_Dec() when dec != null:
return dec(_that.hundredths);case AnswerDto_Remainder() when remainder != null:
return remainder(_that.q,_that.r);case AnswerDto_Roots() when roots != null:
return roots(_that.a,_that.b);case AnswerDto_Pair() when pair != null:
return pair(_that.x,_that.y);case AnswerDto_Ineq() when ineq != null:
return ineq(_that.op,_that.n);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( PlatformInt64 n)  int,required TResult Function( PlatformInt64 n,  PlatformInt64 d)  frac,required TResult Function( PlatformInt64 hundredths)  dec,required TResult Function( PlatformInt64 q,  PlatformInt64 r)  remainder,required TResult Function( PlatformInt64 a,  PlatformInt64 b)  roots,required TResult Function( PlatformInt64 x,  PlatformInt64 y)  pair,required TResult Function( String op,  PlatformInt64 n)  ineq,}) {final _that = this;
switch (_that) {
case AnswerDto_Int():
return int(_that.n);case AnswerDto_Frac():
return frac(_that.n,_that.d);case AnswerDto_Dec():
return dec(_that.hundredths);case AnswerDto_Remainder():
return remainder(_that.q,_that.r);case AnswerDto_Roots():
return roots(_that.a,_that.b);case AnswerDto_Pair():
return pair(_that.x,_that.y);case AnswerDto_Ineq():
return ineq(_that.op,_that.n);}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( PlatformInt64 n)?  int,TResult? Function( PlatformInt64 n,  PlatformInt64 d)?  frac,TResult? Function( PlatformInt64 hundredths)?  dec,TResult? Function( PlatformInt64 q,  PlatformInt64 r)?  remainder,TResult? Function( PlatformInt64 a,  PlatformInt64 b)?  roots,TResult? Function( PlatformInt64 x,  PlatformInt64 y)?  pair,TResult? Function( String op,  PlatformInt64 n)?  ineq,}) {final _that = this;
switch (_that) {
case AnswerDto_Int() when int != null:
return int(_that.n);case AnswerDto_Frac() when frac != null:
return frac(_that.n,_that.d);case AnswerDto_Dec() when dec != null:
return dec(_that.hundredths);case AnswerDto_Remainder() when remainder != null:
return remainder(_that.q,_that.r);case AnswerDto_Roots() when roots != null:
return roots(_that.a,_that.b);case AnswerDto_Pair() when pair != null:
return pair(_that.x,_that.y);case AnswerDto_Ineq() when ineq != null:
return ineq(_that.op,_that.n);case _:
  return null;

}
}

}

/// @nodoc


class AnswerDto_Int extends AnswerDto {
  const AnswerDto_Int({required this.n}): super._();
  

 final  PlatformInt64 n;

/// Create a copy of AnswerDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$AnswerDto_IntCopyWith<AnswerDto_Int> get copyWith => _$AnswerDto_IntCopyWithImpl<AnswerDto_Int>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is AnswerDto_Int&&(identical(other.n, n) || other.n == n));
}


@override
int get hashCode {
    return Object.hash(runtimeType,n);
}

@override
String toString() {
    return 'AnswerDto.int(n: $n)';
}


}

/// @nodoc
abstract mixin class $AnswerDto_IntCopyWith<$Res> implements $AnswerDtoCopyWith<$Res> {
  factory $AnswerDto_IntCopyWith(AnswerDto_Int value, $Res Function(AnswerDto_Int) _then) = _$AnswerDto_IntCopyWithImpl;
@useResult
$Res call({
 PlatformInt64 n
});




}
/// @nodoc
class _$AnswerDto_IntCopyWithImpl<$Res>
    implements $AnswerDto_IntCopyWith<$Res> {
  _$AnswerDto_IntCopyWithImpl(this._self, this._then);

  final AnswerDto_Int _self;
  final $Res Function(AnswerDto_Int) _then;

/// Create a copy of AnswerDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? n = null,}) {
  return _then(AnswerDto_Int(
n: null == n ? _self.n : n // ignore: cast_nullable_to_non_nullable
as PlatformInt64,
  ));
}


}

/// @nodoc


class AnswerDto_Frac extends AnswerDto {
  const AnswerDto_Frac({required this.n, required this.d}): super._();
  

 final  PlatformInt64 n;
 final  PlatformInt64 d;

/// Create a copy of AnswerDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$AnswerDto_FracCopyWith<AnswerDto_Frac> get copyWith => _$AnswerDto_FracCopyWithImpl<AnswerDto_Frac>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is AnswerDto_Frac&&(identical(other.n, n) || other.n == n)&&(identical(other.d, d) || other.d == d));
}


@override
int get hashCode {
    return Object.hash(runtimeType,n,d);
}

@override
String toString() {
    return 'AnswerDto.frac(n: $n, d: $d)';
}


}

/// @nodoc
abstract mixin class $AnswerDto_FracCopyWith<$Res> implements $AnswerDtoCopyWith<$Res> {
  factory $AnswerDto_FracCopyWith(AnswerDto_Frac value, $Res Function(AnswerDto_Frac) _then) = _$AnswerDto_FracCopyWithImpl;
@useResult
$Res call({
 PlatformInt64 n, PlatformInt64 d
});




}
/// @nodoc
class _$AnswerDto_FracCopyWithImpl<$Res>
    implements $AnswerDto_FracCopyWith<$Res> {
  _$AnswerDto_FracCopyWithImpl(this._self, this._then);

  final AnswerDto_Frac _self;
  final $Res Function(AnswerDto_Frac) _then;

/// Create a copy of AnswerDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? n = null,Object? d = null,}) {
  return _then(AnswerDto_Frac(
n: null == n ? _self.n : n // ignore: cast_nullable_to_non_nullable
as PlatformInt64,d: null == d ? _self.d : d // ignore: cast_nullable_to_non_nullable
as PlatformInt64,
  ));
}


}

/// @nodoc


class AnswerDto_Dec extends AnswerDto {
  const AnswerDto_Dec({required this.hundredths}): super._();
  

 final  PlatformInt64 hundredths;

/// Create a copy of AnswerDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$AnswerDto_DecCopyWith<AnswerDto_Dec> get copyWith => _$AnswerDto_DecCopyWithImpl<AnswerDto_Dec>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is AnswerDto_Dec&&(identical(other.hundredths, hundredths) || other.hundredths == hundredths));
}


@override
int get hashCode {
    return Object.hash(runtimeType,hundredths);
}

@override
String toString() {
    return 'AnswerDto.dec(hundredths: $hundredths)';
}


}

/// @nodoc
abstract mixin class $AnswerDto_DecCopyWith<$Res> implements $AnswerDtoCopyWith<$Res> {
  factory $AnswerDto_DecCopyWith(AnswerDto_Dec value, $Res Function(AnswerDto_Dec) _then) = _$AnswerDto_DecCopyWithImpl;
@useResult
$Res call({
 PlatformInt64 hundredths
});




}
/// @nodoc
class _$AnswerDto_DecCopyWithImpl<$Res>
    implements $AnswerDto_DecCopyWith<$Res> {
  _$AnswerDto_DecCopyWithImpl(this._self, this._then);

  final AnswerDto_Dec _self;
  final $Res Function(AnswerDto_Dec) _then;

/// Create a copy of AnswerDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? hundredths = null,}) {
  return _then(AnswerDto_Dec(
hundredths: null == hundredths ? _self.hundredths : hundredths // ignore: cast_nullable_to_non_nullable
as PlatformInt64,
  ));
}


}

/// @nodoc


class AnswerDto_Remainder extends AnswerDto {
  const AnswerDto_Remainder({required this.q, required this.r}): super._();
  

 final  PlatformInt64 q;
 final  PlatformInt64 r;

/// Create a copy of AnswerDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$AnswerDto_RemainderCopyWith<AnswerDto_Remainder> get copyWith => _$AnswerDto_RemainderCopyWithImpl<AnswerDto_Remainder>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is AnswerDto_Remainder&&(identical(other.q, q) || other.q == q)&&(identical(other.r, r) || other.r == r));
}


@override
int get hashCode {
    return Object.hash(runtimeType,q,r);
}

@override
String toString() {
    return 'AnswerDto.remainder(q: $q, r: $r)';
}


}

/// @nodoc
abstract mixin class $AnswerDto_RemainderCopyWith<$Res> implements $AnswerDtoCopyWith<$Res> {
  factory $AnswerDto_RemainderCopyWith(AnswerDto_Remainder value, $Res Function(AnswerDto_Remainder) _then) = _$AnswerDto_RemainderCopyWithImpl;
@useResult
$Res call({
 PlatformInt64 q, PlatformInt64 r
});




}
/// @nodoc
class _$AnswerDto_RemainderCopyWithImpl<$Res>
    implements $AnswerDto_RemainderCopyWith<$Res> {
  _$AnswerDto_RemainderCopyWithImpl(this._self, this._then);

  final AnswerDto_Remainder _self;
  final $Res Function(AnswerDto_Remainder) _then;

/// Create a copy of AnswerDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? q = null,Object? r = null,}) {
  return _then(AnswerDto_Remainder(
q: null == q ? _self.q : q // ignore: cast_nullable_to_non_nullable
as PlatformInt64,r: null == r ? _self.r : r // ignore: cast_nullable_to_non_nullable
as PlatformInt64,
  ));
}


}

/// @nodoc


class AnswerDto_Roots extends AnswerDto {
  const AnswerDto_Roots({required this.a, required this.b}): super._();
  

 final  PlatformInt64 a;
 final  PlatformInt64 b;

/// Create a copy of AnswerDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$AnswerDto_RootsCopyWith<AnswerDto_Roots> get copyWith => _$AnswerDto_RootsCopyWithImpl<AnswerDto_Roots>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is AnswerDto_Roots&&(identical(other.a, a) || other.a == a)&&(identical(other.b, b) || other.b == b));
}


@override
int get hashCode {
    return Object.hash(runtimeType,a,b);
}

@override
String toString() {
    return 'AnswerDto.roots(a: $a, b: $b)';
}


}

/// @nodoc
abstract mixin class $AnswerDto_RootsCopyWith<$Res> implements $AnswerDtoCopyWith<$Res> {
  factory $AnswerDto_RootsCopyWith(AnswerDto_Roots value, $Res Function(AnswerDto_Roots) _then) = _$AnswerDto_RootsCopyWithImpl;
@useResult
$Res call({
 PlatformInt64 a, PlatformInt64 b
});




}
/// @nodoc
class _$AnswerDto_RootsCopyWithImpl<$Res>
    implements $AnswerDto_RootsCopyWith<$Res> {
  _$AnswerDto_RootsCopyWithImpl(this._self, this._then);

  final AnswerDto_Roots _self;
  final $Res Function(AnswerDto_Roots) _then;

/// Create a copy of AnswerDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? a = null,Object? b = null,}) {
  return _then(AnswerDto_Roots(
a: null == a ? _self.a : a // ignore: cast_nullable_to_non_nullable
as PlatformInt64,b: null == b ? _self.b : b // ignore: cast_nullable_to_non_nullable
as PlatformInt64,
  ));
}


}

/// @nodoc


class AnswerDto_Pair extends AnswerDto {
  const AnswerDto_Pair({required this.x, required this.y}): super._();
  

 final  PlatformInt64 x;
 final  PlatformInt64 y;

/// Create a copy of AnswerDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$AnswerDto_PairCopyWith<AnswerDto_Pair> get copyWith => _$AnswerDto_PairCopyWithImpl<AnswerDto_Pair>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is AnswerDto_Pair&&(identical(other.x, x) || other.x == x)&&(identical(other.y, y) || other.y == y));
}


@override
int get hashCode {
    return Object.hash(runtimeType,x,y);
}

@override
String toString() {
    return 'AnswerDto.pair(x: $x, y: $y)';
}


}

/// @nodoc
abstract mixin class $AnswerDto_PairCopyWith<$Res> implements $AnswerDtoCopyWith<$Res> {
  factory $AnswerDto_PairCopyWith(AnswerDto_Pair value, $Res Function(AnswerDto_Pair) _then) = _$AnswerDto_PairCopyWithImpl;
@useResult
$Res call({
 PlatformInt64 x, PlatformInt64 y
});




}
/// @nodoc
class _$AnswerDto_PairCopyWithImpl<$Res>
    implements $AnswerDto_PairCopyWith<$Res> {
  _$AnswerDto_PairCopyWithImpl(this._self, this._then);

  final AnswerDto_Pair _self;
  final $Res Function(AnswerDto_Pair) _then;

/// Create a copy of AnswerDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? x = null,Object? y = null,}) {
  return _then(AnswerDto_Pair(
x: null == x ? _self.x : x // ignore: cast_nullable_to_non_nullable
as PlatformInt64,y: null == y ? _self.y : y // ignore: cast_nullable_to_non_nullable
as PlatformInt64,
  ));
}


}

/// @nodoc


class AnswerDto_Ineq extends AnswerDto {
  const AnswerDto_Ineq({required this.op, required this.n}): super._();
  

 final  String op;
 final  PlatformInt64 n;

/// Create a copy of AnswerDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$AnswerDto_IneqCopyWith<AnswerDto_Ineq> get copyWith => _$AnswerDto_IneqCopyWithImpl<AnswerDto_Ineq>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is AnswerDto_Ineq&&(identical(other.op, op) || other.op == op)&&(identical(other.n, n) || other.n == n));
}


@override
int get hashCode {
    return Object.hash(runtimeType,op,n);
}

@override
String toString() {
    return 'AnswerDto.ineq(op: $op, n: $n)';
}


}

/// @nodoc
abstract mixin class $AnswerDto_IneqCopyWith<$Res> implements $AnswerDtoCopyWith<$Res> {
  factory $AnswerDto_IneqCopyWith(AnswerDto_Ineq value, $Res Function(AnswerDto_Ineq) _then) = _$AnswerDto_IneqCopyWithImpl;
@useResult
$Res call({
 String op, PlatformInt64 n
});




}
/// @nodoc
class _$AnswerDto_IneqCopyWithImpl<$Res>
    implements $AnswerDto_IneqCopyWith<$Res> {
  _$AnswerDto_IneqCopyWithImpl(this._self, this._then);

  final AnswerDto_Ineq _self;
  final $Res Function(AnswerDto_Ineq) _then;

/// Create a copy of AnswerDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? op = null,Object? n = null,}) {
  return _then(AnswerDto_Ineq(
op: null == op ? _self.op : op // ignore: cast_nullable_to_non_nullable
as String,n: null == n ? _self.n : n // ignore: cast_nullable_to_non_nullable
as PlatformInt64,
  ));
}


}

/// @nodoc
mixin _$FigureDto {





@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is FigureDto);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
    return 'FigureDto()';
}


}

/// @nodoc
class $FigureDtoCopyWith<$Res>  {
$FigureDtoCopyWith(FigureDto _, $Res Function(FigureDto) __);
}


/// Adds pattern-matching-related methods to [FigureDto].
extension FigureDtoPatterns on FigureDto {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( FigureDto_Right value)?  right,TResult Function( FigureDto_Sas value)?  sas,TResult Function( FigureDto_Unit value)?  unit,required TResult orElse(),}){
final _that = this;
switch (_that) {
case FigureDto_Right() when right != null:
return right(_that);case FigureDto_Sas() when sas != null:
return sas(_that);case FigureDto_Unit() when unit != null:
return unit(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( FigureDto_Right value)  right,required TResult Function( FigureDto_Sas value)  sas,required TResult Function( FigureDto_Unit value)  unit,}){
final _that = this;
switch (_that) {
case FigureDto_Right():
return right(_that);case FigureDto_Sas():
return sas(_that);case FigureDto_Unit():
return unit(_that);}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( FigureDto_Right value)?  right,TResult? Function( FigureDto_Sas value)?  sas,TResult? Function( FigureDto_Unit value)?  unit,}){
final _that = this;
switch (_that) {
case FigureDto_Right() when right != null:
return right(_that);case FigureDto_Sas() when sas != null:
return sas(_that);case FigureDto_Unit() when unit != null:
return unit(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( double adj,  double opp,  String angle,  String adjLabel,  String oppLabel,  String hypLabel)?  right,TResult Function( double a,  double b,  double deg,  String aLabel,  String bLabel,  String angle,  String third)?  sas,TResult Function( double deg,  String label)?  unit,required TResult orElse(),}) {final _that = this;
switch (_that) {
case FigureDto_Right() when right != null:
return right(_that.adj,_that.opp,_that.angle,_that.adjLabel,_that.oppLabel,_that.hypLabel);case FigureDto_Sas() when sas != null:
return sas(_that.a,_that.b,_that.deg,_that.aLabel,_that.bLabel,_that.angle,_that.third);case FigureDto_Unit() when unit != null:
return unit(_that.deg,_that.label);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( double adj,  double opp,  String angle,  String adjLabel,  String oppLabel,  String hypLabel)  right,required TResult Function( double a,  double b,  double deg,  String aLabel,  String bLabel,  String angle,  String third)  sas,required TResult Function( double deg,  String label)  unit,}) {final _that = this;
switch (_that) {
case FigureDto_Right():
return right(_that.adj,_that.opp,_that.angle,_that.adjLabel,_that.oppLabel,_that.hypLabel);case FigureDto_Sas():
return sas(_that.a,_that.b,_that.deg,_that.aLabel,_that.bLabel,_that.angle,_that.third);case FigureDto_Unit():
return unit(_that.deg,_that.label);}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( double adj,  double opp,  String angle,  String adjLabel,  String oppLabel,  String hypLabel)?  right,TResult? Function( double a,  double b,  double deg,  String aLabel,  String bLabel,  String angle,  String third)?  sas,TResult? Function( double deg,  String label)?  unit,}) {final _that = this;
switch (_that) {
case FigureDto_Right() when right != null:
return right(_that.adj,_that.opp,_that.angle,_that.adjLabel,_that.oppLabel,_that.hypLabel);case FigureDto_Sas() when sas != null:
return sas(_that.a,_that.b,_that.deg,_that.aLabel,_that.bLabel,_that.angle,_that.third);case FigureDto_Unit() when unit != null:
return unit(_that.deg,_that.label);case _:
  return null;

}
}

}

/// @nodoc


class FigureDto_Right extends FigureDto {
  const FigureDto_Right({required this.adj, required this.opp, required this.angle, required this.adjLabel, required this.oppLabel, required this.hypLabel}): super._();
  

 final  double adj;
 final  double opp;
 final  String angle;
 final  String adjLabel;
 final  String oppLabel;
 final  String hypLabel;

/// Create a copy of FigureDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$FigureDto_RightCopyWith<FigureDto_Right> get copyWith => _$FigureDto_RightCopyWithImpl<FigureDto_Right>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is FigureDto_Right&&(identical(other.adj, adj) || other.adj == adj)&&(identical(other.opp, opp) || other.opp == opp)&&(identical(other.angle, angle) || other.angle == angle)&&(identical(other.adjLabel, adjLabel) || other.adjLabel == adjLabel)&&(identical(other.oppLabel, oppLabel) || other.oppLabel == oppLabel)&&(identical(other.hypLabel, hypLabel) || other.hypLabel == hypLabel));
}


@override
int get hashCode {
    return Object.hash(runtimeType,adj,opp,angle,adjLabel,oppLabel,hypLabel);
}

@override
String toString() {
    return 'FigureDto.right(adj: $adj, opp: $opp, angle: $angle, adjLabel: $adjLabel, oppLabel: $oppLabel, hypLabel: $hypLabel)';
}


}

/// @nodoc
abstract mixin class $FigureDto_RightCopyWith<$Res> implements $FigureDtoCopyWith<$Res> {
  factory $FigureDto_RightCopyWith(FigureDto_Right value, $Res Function(FigureDto_Right) _then) = _$FigureDto_RightCopyWithImpl;
@useResult
$Res call({
 double adj, double opp, String angle, String adjLabel, String oppLabel, String hypLabel
});




}
/// @nodoc
class _$FigureDto_RightCopyWithImpl<$Res>
    implements $FigureDto_RightCopyWith<$Res> {
  _$FigureDto_RightCopyWithImpl(this._self, this._then);

  final FigureDto_Right _self;
  final $Res Function(FigureDto_Right) _then;

/// Create a copy of FigureDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? adj = null,Object? opp = null,Object? angle = null,Object? adjLabel = null,Object? oppLabel = null,Object? hypLabel = null,}) {
  return _then(FigureDto_Right(
adj: null == adj ? _self.adj : adj // ignore: cast_nullable_to_non_nullable
as double,opp: null == opp ? _self.opp : opp // ignore: cast_nullable_to_non_nullable
as double,angle: null == angle ? _self.angle : angle // ignore: cast_nullable_to_non_nullable
as String,adjLabel: null == adjLabel ? _self.adjLabel : adjLabel // ignore: cast_nullable_to_non_nullable
as String,oppLabel: null == oppLabel ? _self.oppLabel : oppLabel // ignore: cast_nullable_to_non_nullable
as String,hypLabel: null == hypLabel ? _self.hypLabel : hypLabel // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class FigureDto_Sas extends FigureDto {
  const FigureDto_Sas({required this.a, required this.b, required this.deg, required this.aLabel, required this.bLabel, required this.angle, required this.third}): super._();
  

 final  double a;
 final  double b;
 final  double deg;
 final  String aLabel;
 final  String bLabel;
 final  String angle;
 final  String third;

/// Create a copy of FigureDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$FigureDto_SasCopyWith<FigureDto_Sas> get copyWith => _$FigureDto_SasCopyWithImpl<FigureDto_Sas>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is FigureDto_Sas&&(identical(other.a, a) || other.a == a)&&(identical(other.b, b) || other.b == b)&&(identical(other.deg, deg) || other.deg == deg)&&(identical(other.aLabel, aLabel) || other.aLabel == aLabel)&&(identical(other.bLabel, bLabel) || other.bLabel == bLabel)&&(identical(other.angle, angle) || other.angle == angle)&&(identical(other.third, third) || other.third == third));
}


@override
int get hashCode {
    return Object.hash(runtimeType,a,b,deg,aLabel,bLabel,angle,third);
}

@override
String toString() {
    return 'FigureDto.sas(a: $a, b: $b, deg: $deg, aLabel: $aLabel, bLabel: $bLabel, angle: $angle, third: $third)';
}


}

/// @nodoc
abstract mixin class $FigureDto_SasCopyWith<$Res> implements $FigureDtoCopyWith<$Res> {
  factory $FigureDto_SasCopyWith(FigureDto_Sas value, $Res Function(FigureDto_Sas) _then) = _$FigureDto_SasCopyWithImpl;
@useResult
$Res call({
 double a, double b, double deg, String aLabel, String bLabel, String angle, String third
});




}
/// @nodoc
class _$FigureDto_SasCopyWithImpl<$Res>
    implements $FigureDto_SasCopyWith<$Res> {
  _$FigureDto_SasCopyWithImpl(this._self, this._then);

  final FigureDto_Sas _self;
  final $Res Function(FigureDto_Sas) _then;

/// Create a copy of FigureDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? a = null,Object? b = null,Object? deg = null,Object? aLabel = null,Object? bLabel = null,Object? angle = null,Object? third = null,}) {
  return _then(FigureDto_Sas(
a: null == a ? _self.a : a // ignore: cast_nullable_to_non_nullable
as double,b: null == b ? _self.b : b // ignore: cast_nullable_to_non_nullable
as double,deg: null == deg ? _self.deg : deg // ignore: cast_nullable_to_non_nullable
as double,aLabel: null == aLabel ? _self.aLabel : aLabel // ignore: cast_nullable_to_non_nullable
as String,bLabel: null == bLabel ? _self.bLabel : bLabel // ignore: cast_nullable_to_non_nullable
as String,angle: null == angle ? _self.angle : angle // ignore: cast_nullable_to_non_nullable
as String,third: null == third ? _self.third : third // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class FigureDto_Unit extends FigureDto {
  const FigureDto_Unit({required this.deg, required this.label}): super._();
  

 final  double deg;
 final  String label;

/// Create a copy of FigureDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$FigureDto_UnitCopyWith<FigureDto_Unit> get copyWith => _$FigureDto_UnitCopyWithImpl<FigureDto_Unit>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is FigureDto_Unit&&(identical(other.deg, deg) || other.deg == deg)&&(identical(other.label, label) || other.label == label));
}


@override
int get hashCode {
    return Object.hash(runtimeType,deg,label);
}

@override
String toString() {
    return 'FigureDto.unit(deg: $deg, label: $label)';
}


}

/// @nodoc
abstract mixin class $FigureDto_UnitCopyWith<$Res> implements $FigureDtoCopyWith<$Res> {
  factory $FigureDto_UnitCopyWith(FigureDto_Unit value, $Res Function(FigureDto_Unit) _then) = _$FigureDto_UnitCopyWithImpl;
@useResult
$Res call({
 double deg, String label
});




}
/// @nodoc
class _$FigureDto_UnitCopyWithImpl<$Res>
    implements $FigureDto_UnitCopyWith<$Res> {
  _$FigureDto_UnitCopyWithImpl(this._self, this._then);

  final FigureDto_Unit _self;
  final $Res Function(FigureDto_Unit) _then;

/// Create a copy of FigureDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? deg = null,Object? label = null,}) {
  return _then(FigureDto_Unit(
deg: null == deg ? _self.deg : deg // ignore: cast_nullable_to_non_nullable
as double,label: null == label ? _self.label : label // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc
mixin _$VerdictDto {





@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is VerdictDto);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
    return 'VerdictDto()';
}


}

/// @nodoc
class $VerdictDtoCopyWith<$Res>  {
$VerdictDtoCopyWith(VerdictDto _, $Res Function(VerdictDto) __);
}


/// Adds pattern-matching-related methods to [VerdictDto].
extension VerdictDtoPatterns on VerdictDto {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( VerdictDto_Right value)?  right,TResult Function( VerdictDto_Almost value)?  almost,TResult Function( VerdictDto_Wrong value)?  wrong,TResult Function( VerdictDto_Unreadable value)?  unreadable,required TResult orElse(),}){
final _that = this;
switch (_that) {
case VerdictDto_Right() when right != null:
return right(_that);case VerdictDto_Almost() when almost != null:
return almost(_that);case VerdictDto_Wrong() when wrong != null:
return wrong(_that);case VerdictDto_Unreadable() when unreadable != null:
return unreadable(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( VerdictDto_Right value)  right,required TResult Function( VerdictDto_Almost value)  almost,required TResult Function( VerdictDto_Wrong value)  wrong,required TResult Function( VerdictDto_Unreadable value)  unreadable,}){
final _that = this;
switch (_that) {
case VerdictDto_Right():
return right(_that);case VerdictDto_Almost():
return almost(_that);case VerdictDto_Wrong():
return wrong(_that);case VerdictDto_Unreadable():
return unreadable(_that);}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( VerdictDto_Right value)?  right,TResult? Function( VerdictDto_Almost value)?  almost,TResult? Function( VerdictDto_Wrong value)?  wrong,TResult? Function( VerdictDto_Unreadable value)?  unreadable,}){
final _that = this;
switch (_that) {
case VerdictDto_Right() when right != null:
return right(_that);case VerdictDto_Almost() when almost != null:
return almost(_that);case VerdictDto_Wrong() when wrong != null:
return wrong(_that);case VerdictDto_Unreadable() when unreadable != null:
return unreadable(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function()?  right,TResult Function( String message)?  almost,TResult Function()?  wrong,TResult Function( String message)?  unreadable,required TResult orElse(),}) {final _that = this;
switch (_that) {
case VerdictDto_Right() when right != null:
return right();case VerdictDto_Almost() when almost != null:
return almost(_that.message);case VerdictDto_Wrong() when wrong != null:
return wrong();case VerdictDto_Unreadable() when unreadable != null:
return unreadable(_that.message);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function()  right,required TResult Function( String message)  almost,required TResult Function()  wrong,required TResult Function( String message)  unreadable,}) {final _that = this;
switch (_that) {
case VerdictDto_Right():
return right();case VerdictDto_Almost():
return almost(_that.message);case VerdictDto_Wrong():
return wrong();case VerdictDto_Unreadable():
return unreadable(_that.message);}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function()?  right,TResult? Function( String message)?  almost,TResult? Function()?  wrong,TResult? Function( String message)?  unreadable,}) {final _that = this;
switch (_that) {
case VerdictDto_Right() when right != null:
return right();case VerdictDto_Almost() when almost != null:
return almost(_that.message);case VerdictDto_Wrong() when wrong != null:
return wrong();case VerdictDto_Unreadable() when unreadable != null:
return unreadable(_that.message);case _:
  return null;

}
}

}

/// @nodoc


class VerdictDto_Right extends VerdictDto {
  const VerdictDto_Right(): super._();
  






@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is VerdictDto_Right);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
    return 'VerdictDto.right()';
}


}




/// @nodoc


class VerdictDto_Almost extends VerdictDto {
  const VerdictDto_Almost({required this.message}): super._();
  

 final  String message;

/// Create a copy of VerdictDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$VerdictDto_AlmostCopyWith<VerdictDto_Almost> get copyWith => _$VerdictDto_AlmostCopyWithImpl<VerdictDto_Almost>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is VerdictDto_Almost&&(identical(other.message, message) || other.message == message));
}


@override
int get hashCode {
    return Object.hash(runtimeType,message);
}

@override
String toString() {
    return 'VerdictDto.almost(message: $message)';
}


}

/// @nodoc
abstract mixin class $VerdictDto_AlmostCopyWith<$Res> implements $VerdictDtoCopyWith<$Res> {
  factory $VerdictDto_AlmostCopyWith(VerdictDto_Almost value, $Res Function(VerdictDto_Almost) _then) = _$VerdictDto_AlmostCopyWithImpl;
@useResult
$Res call({
 String message
});




}
/// @nodoc
class _$VerdictDto_AlmostCopyWithImpl<$Res>
    implements $VerdictDto_AlmostCopyWith<$Res> {
  _$VerdictDto_AlmostCopyWithImpl(this._self, this._then);

  final VerdictDto_Almost _self;
  final $Res Function(VerdictDto_Almost) _then;

/// Create a copy of VerdictDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? message = null,}) {
  return _then(VerdictDto_Almost(
message: null == message ? _self.message : message // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class VerdictDto_Wrong extends VerdictDto {
  const VerdictDto_Wrong(): super._();
  






@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is VerdictDto_Wrong);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
    return 'VerdictDto.wrong()';
}


}




/// @nodoc


class VerdictDto_Unreadable extends VerdictDto {
  const VerdictDto_Unreadable({required this.message}): super._();
  

 final  String message;

/// Create a copy of VerdictDto
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$VerdictDto_UnreadableCopyWith<VerdictDto_Unreadable> get copyWith => _$VerdictDto_UnreadableCopyWithImpl<VerdictDto_Unreadable>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is VerdictDto_Unreadable&&(identical(other.message, message) || other.message == message));
}


@override
int get hashCode {
    return Object.hash(runtimeType,message);
}

@override
String toString() {
    return 'VerdictDto.unreadable(message: $message)';
}


}

/// @nodoc
abstract mixin class $VerdictDto_UnreadableCopyWith<$Res> implements $VerdictDtoCopyWith<$Res> {
  factory $VerdictDto_UnreadableCopyWith(VerdictDto_Unreadable value, $Res Function(VerdictDto_Unreadable) _then) = _$VerdictDto_UnreadableCopyWithImpl;
@useResult
$Res call({
 String message
});




}
/// @nodoc
class _$VerdictDto_UnreadableCopyWithImpl<$Res>
    implements $VerdictDto_UnreadableCopyWith<$Res> {
  _$VerdictDto_UnreadableCopyWithImpl(this._self, this._then);

  final VerdictDto_Unreadable _self;
  final $Res Function(VerdictDto_Unreadable) _then;

/// Create a copy of VerdictDto
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? message = null,}) {
  return _then(VerdictDto_Unreadable(
message: null == message ? _self.message : message // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

// dart format on
