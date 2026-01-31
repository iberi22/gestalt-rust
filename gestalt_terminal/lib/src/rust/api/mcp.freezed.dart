// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'mcp.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
  'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models',
);

/// @nodoc
mixin _$McpComponent {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String title, String content) card,
    required TResult Function(String label, String actionId) button,
    required TResult Function(String content) markdown,
    required TResult Function(List<McpComponent> children) row,
    required TResult Function(List<McpComponent> children) column,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String title, String content)? card,
    TResult? Function(String label, String actionId)? button,
    TResult? Function(String content)? markdown,
    TResult? Function(List<McpComponent> children)? row,
    TResult? Function(List<McpComponent> children)? column,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String title, String content)? card,
    TResult Function(String label, String actionId)? button,
    TResult Function(String content)? markdown,
    TResult Function(List<McpComponent> children)? row,
    TResult Function(List<McpComponent> children)? column,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(McpComponent_Card value) card,
    required TResult Function(McpComponent_Button value) button,
    required TResult Function(McpComponent_Markdown value) markdown,
    required TResult Function(McpComponent_Row value) row,
    required TResult Function(McpComponent_Column value) column,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(McpComponent_Card value)? card,
    TResult? Function(McpComponent_Button value)? button,
    TResult? Function(McpComponent_Markdown value)? markdown,
    TResult? Function(McpComponent_Row value)? row,
    TResult? Function(McpComponent_Column value)? column,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(McpComponent_Card value)? card,
    TResult Function(McpComponent_Button value)? button,
    TResult Function(McpComponent_Markdown value)? markdown,
    TResult Function(McpComponent_Row value)? row,
    TResult Function(McpComponent_Column value)? column,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $McpComponentCopyWith<$Res> {
  factory $McpComponentCopyWith(
    McpComponent value,
    $Res Function(McpComponent) then,
  ) = _$McpComponentCopyWithImpl<$Res, McpComponent>;
}

/// @nodoc
class _$McpComponentCopyWithImpl<$Res, $Val extends McpComponent>
    implements $McpComponentCopyWith<$Res> {
  _$McpComponentCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of McpComponent
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
abstract class _$$McpComponent_CardImplCopyWith<$Res> {
  factory _$$McpComponent_CardImplCopyWith(
    _$McpComponent_CardImpl value,
    $Res Function(_$McpComponent_CardImpl) then,
  ) = __$$McpComponent_CardImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String title, String content});
}

/// @nodoc
class __$$McpComponent_CardImplCopyWithImpl<$Res>
    extends _$McpComponentCopyWithImpl<$Res, _$McpComponent_CardImpl>
    implements _$$McpComponent_CardImplCopyWith<$Res> {
  __$$McpComponent_CardImplCopyWithImpl(
    _$McpComponent_CardImpl _value,
    $Res Function(_$McpComponent_CardImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of McpComponent
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? title = null, Object? content = null}) {
    return _then(
      _$McpComponent_CardImpl(
        title: null == title
            ? _value.title
            : title // ignore: cast_nullable_to_non_nullable
                  as String,
        content: null == content
            ? _value.content
            : content // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$McpComponent_CardImpl extends McpComponent_Card {
  const _$McpComponent_CardImpl({required this.title, required this.content})
    : super._();

  @override
  final String title;
  @override
  final String content;

  @override
  String toString() {
    return 'McpComponent.card(title: $title, content: $content)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$McpComponent_CardImpl &&
            (identical(other.title, title) || other.title == title) &&
            (identical(other.content, content) || other.content == content));
  }

  @override
  int get hashCode => Object.hash(runtimeType, title, content);

  /// Create a copy of McpComponent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$McpComponent_CardImplCopyWith<_$McpComponent_CardImpl> get copyWith =>
      __$$McpComponent_CardImplCopyWithImpl<_$McpComponent_CardImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String title, String content) card,
    required TResult Function(String label, String actionId) button,
    required TResult Function(String content) markdown,
    required TResult Function(List<McpComponent> children) row,
    required TResult Function(List<McpComponent> children) column,
  }) {
    return card(title, content);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String title, String content)? card,
    TResult? Function(String label, String actionId)? button,
    TResult? Function(String content)? markdown,
    TResult? Function(List<McpComponent> children)? row,
    TResult? Function(List<McpComponent> children)? column,
  }) {
    return card?.call(title, content);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String title, String content)? card,
    TResult Function(String label, String actionId)? button,
    TResult Function(String content)? markdown,
    TResult Function(List<McpComponent> children)? row,
    TResult Function(List<McpComponent> children)? column,
    required TResult orElse(),
  }) {
    if (card != null) {
      return card(title, content);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(McpComponent_Card value) card,
    required TResult Function(McpComponent_Button value) button,
    required TResult Function(McpComponent_Markdown value) markdown,
    required TResult Function(McpComponent_Row value) row,
    required TResult Function(McpComponent_Column value) column,
  }) {
    return card(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(McpComponent_Card value)? card,
    TResult? Function(McpComponent_Button value)? button,
    TResult? Function(McpComponent_Markdown value)? markdown,
    TResult? Function(McpComponent_Row value)? row,
    TResult? Function(McpComponent_Column value)? column,
  }) {
    return card?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(McpComponent_Card value)? card,
    TResult Function(McpComponent_Button value)? button,
    TResult Function(McpComponent_Markdown value)? markdown,
    TResult Function(McpComponent_Row value)? row,
    TResult Function(McpComponent_Column value)? column,
    required TResult orElse(),
  }) {
    if (card != null) {
      return card(this);
    }
    return orElse();
  }
}

abstract class McpComponent_Card extends McpComponent {
  const factory McpComponent_Card({
    required final String title,
    required final String content,
  }) = _$McpComponent_CardImpl;
  const McpComponent_Card._() : super._();

  String get title;
  String get content;

  /// Create a copy of McpComponent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$McpComponent_CardImplCopyWith<_$McpComponent_CardImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$McpComponent_ButtonImplCopyWith<$Res> {
  factory _$$McpComponent_ButtonImplCopyWith(
    _$McpComponent_ButtonImpl value,
    $Res Function(_$McpComponent_ButtonImpl) then,
  ) = __$$McpComponent_ButtonImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String label, String actionId});
}

/// @nodoc
class __$$McpComponent_ButtonImplCopyWithImpl<$Res>
    extends _$McpComponentCopyWithImpl<$Res, _$McpComponent_ButtonImpl>
    implements _$$McpComponent_ButtonImplCopyWith<$Res> {
  __$$McpComponent_ButtonImplCopyWithImpl(
    _$McpComponent_ButtonImpl _value,
    $Res Function(_$McpComponent_ButtonImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of McpComponent
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? label = null, Object? actionId = null}) {
    return _then(
      _$McpComponent_ButtonImpl(
        label: null == label
            ? _value.label
            : label // ignore: cast_nullable_to_non_nullable
                  as String,
        actionId: null == actionId
            ? _value.actionId
            : actionId // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$McpComponent_ButtonImpl extends McpComponent_Button {
  const _$McpComponent_ButtonImpl({required this.label, required this.actionId})
    : super._();

  @override
  final String label;
  @override
  final String actionId;

  @override
  String toString() {
    return 'McpComponent.button(label: $label, actionId: $actionId)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$McpComponent_ButtonImpl &&
            (identical(other.label, label) || other.label == label) &&
            (identical(other.actionId, actionId) ||
                other.actionId == actionId));
  }

  @override
  int get hashCode => Object.hash(runtimeType, label, actionId);

  /// Create a copy of McpComponent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$McpComponent_ButtonImplCopyWith<_$McpComponent_ButtonImpl> get copyWith =>
      __$$McpComponent_ButtonImplCopyWithImpl<_$McpComponent_ButtonImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String title, String content) card,
    required TResult Function(String label, String actionId) button,
    required TResult Function(String content) markdown,
    required TResult Function(List<McpComponent> children) row,
    required TResult Function(List<McpComponent> children) column,
  }) {
    return button(label, actionId);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String title, String content)? card,
    TResult? Function(String label, String actionId)? button,
    TResult? Function(String content)? markdown,
    TResult? Function(List<McpComponent> children)? row,
    TResult? Function(List<McpComponent> children)? column,
  }) {
    return button?.call(label, actionId);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String title, String content)? card,
    TResult Function(String label, String actionId)? button,
    TResult Function(String content)? markdown,
    TResult Function(List<McpComponent> children)? row,
    TResult Function(List<McpComponent> children)? column,
    required TResult orElse(),
  }) {
    if (button != null) {
      return button(label, actionId);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(McpComponent_Card value) card,
    required TResult Function(McpComponent_Button value) button,
    required TResult Function(McpComponent_Markdown value) markdown,
    required TResult Function(McpComponent_Row value) row,
    required TResult Function(McpComponent_Column value) column,
  }) {
    return button(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(McpComponent_Card value)? card,
    TResult? Function(McpComponent_Button value)? button,
    TResult? Function(McpComponent_Markdown value)? markdown,
    TResult? Function(McpComponent_Row value)? row,
    TResult? Function(McpComponent_Column value)? column,
  }) {
    return button?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(McpComponent_Card value)? card,
    TResult Function(McpComponent_Button value)? button,
    TResult Function(McpComponent_Markdown value)? markdown,
    TResult Function(McpComponent_Row value)? row,
    TResult Function(McpComponent_Column value)? column,
    required TResult orElse(),
  }) {
    if (button != null) {
      return button(this);
    }
    return orElse();
  }
}

abstract class McpComponent_Button extends McpComponent {
  const factory McpComponent_Button({
    required final String label,
    required final String actionId,
  }) = _$McpComponent_ButtonImpl;
  const McpComponent_Button._() : super._();

  String get label;
  String get actionId;

  /// Create a copy of McpComponent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$McpComponent_ButtonImplCopyWith<_$McpComponent_ButtonImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$McpComponent_MarkdownImplCopyWith<$Res> {
  factory _$$McpComponent_MarkdownImplCopyWith(
    _$McpComponent_MarkdownImpl value,
    $Res Function(_$McpComponent_MarkdownImpl) then,
  ) = __$$McpComponent_MarkdownImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String content});
}

/// @nodoc
class __$$McpComponent_MarkdownImplCopyWithImpl<$Res>
    extends _$McpComponentCopyWithImpl<$Res, _$McpComponent_MarkdownImpl>
    implements _$$McpComponent_MarkdownImplCopyWith<$Res> {
  __$$McpComponent_MarkdownImplCopyWithImpl(
    _$McpComponent_MarkdownImpl _value,
    $Res Function(_$McpComponent_MarkdownImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of McpComponent
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? content = null}) {
    return _then(
      _$McpComponent_MarkdownImpl(
        content: null == content
            ? _value.content
            : content // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$McpComponent_MarkdownImpl extends McpComponent_Markdown {
  const _$McpComponent_MarkdownImpl({required this.content}) : super._();

  @override
  final String content;

  @override
  String toString() {
    return 'McpComponent.markdown(content: $content)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$McpComponent_MarkdownImpl &&
            (identical(other.content, content) || other.content == content));
  }

  @override
  int get hashCode => Object.hash(runtimeType, content);

  /// Create a copy of McpComponent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$McpComponent_MarkdownImplCopyWith<_$McpComponent_MarkdownImpl>
  get copyWith =>
      __$$McpComponent_MarkdownImplCopyWithImpl<_$McpComponent_MarkdownImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String title, String content) card,
    required TResult Function(String label, String actionId) button,
    required TResult Function(String content) markdown,
    required TResult Function(List<McpComponent> children) row,
    required TResult Function(List<McpComponent> children) column,
  }) {
    return markdown(content);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String title, String content)? card,
    TResult? Function(String label, String actionId)? button,
    TResult? Function(String content)? markdown,
    TResult? Function(List<McpComponent> children)? row,
    TResult? Function(List<McpComponent> children)? column,
  }) {
    return markdown?.call(content);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String title, String content)? card,
    TResult Function(String label, String actionId)? button,
    TResult Function(String content)? markdown,
    TResult Function(List<McpComponent> children)? row,
    TResult Function(List<McpComponent> children)? column,
    required TResult orElse(),
  }) {
    if (markdown != null) {
      return markdown(content);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(McpComponent_Card value) card,
    required TResult Function(McpComponent_Button value) button,
    required TResult Function(McpComponent_Markdown value) markdown,
    required TResult Function(McpComponent_Row value) row,
    required TResult Function(McpComponent_Column value) column,
  }) {
    return markdown(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(McpComponent_Card value)? card,
    TResult? Function(McpComponent_Button value)? button,
    TResult? Function(McpComponent_Markdown value)? markdown,
    TResult? Function(McpComponent_Row value)? row,
    TResult? Function(McpComponent_Column value)? column,
  }) {
    return markdown?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(McpComponent_Card value)? card,
    TResult Function(McpComponent_Button value)? button,
    TResult Function(McpComponent_Markdown value)? markdown,
    TResult Function(McpComponent_Row value)? row,
    TResult Function(McpComponent_Column value)? column,
    required TResult orElse(),
  }) {
    if (markdown != null) {
      return markdown(this);
    }
    return orElse();
  }
}

abstract class McpComponent_Markdown extends McpComponent {
  const factory McpComponent_Markdown({required final String content}) =
      _$McpComponent_MarkdownImpl;
  const McpComponent_Markdown._() : super._();

  String get content;

  /// Create a copy of McpComponent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$McpComponent_MarkdownImplCopyWith<_$McpComponent_MarkdownImpl>
  get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$McpComponent_RowImplCopyWith<$Res> {
  factory _$$McpComponent_RowImplCopyWith(
    _$McpComponent_RowImpl value,
    $Res Function(_$McpComponent_RowImpl) then,
  ) = __$$McpComponent_RowImplCopyWithImpl<$Res>;
  @useResult
  $Res call({List<McpComponent> children});
}

/// @nodoc
class __$$McpComponent_RowImplCopyWithImpl<$Res>
    extends _$McpComponentCopyWithImpl<$Res, _$McpComponent_RowImpl>
    implements _$$McpComponent_RowImplCopyWith<$Res> {
  __$$McpComponent_RowImplCopyWithImpl(
    _$McpComponent_RowImpl _value,
    $Res Function(_$McpComponent_RowImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of McpComponent
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? children = null}) {
    return _then(
      _$McpComponent_RowImpl(
        children: null == children
            ? _value._children
            : children // ignore: cast_nullable_to_non_nullable
                  as List<McpComponent>,
      ),
    );
  }
}

/// @nodoc

class _$McpComponent_RowImpl extends McpComponent_Row {
  const _$McpComponent_RowImpl({required final List<McpComponent> children})
    : _children = children,
      super._();

  final List<McpComponent> _children;
  @override
  List<McpComponent> get children {
    if (_children is EqualUnmodifiableListView) return _children;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_children);
  }

  @override
  String toString() {
    return 'McpComponent.row(children: $children)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$McpComponent_RowImpl &&
            const DeepCollectionEquality().equals(other._children, _children));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(_children));

  /// Create a copy of McpComponent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$McpComponent_RowImplCopyWith<_$McpComponent_RowImpl> get copyWith =>
      __$$McpComponent_RowImplCopyWithImpl<_$McpComponent_RowImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String title, String content) card,
    required TResult Function(String label, String actionId) button,
    required TResult Function(String content) markdown,
    required TResult Function(List<McpComponent> children) row,
    required TResult Function(List<McpComponent> children) column,
  }) {
    return row(children);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String title, String content)? card,
    TResult? Function(String label, String actionId)? button,
    TResult? Function(String content)? markdown,
    TResult? Function(List<McpComponent> children)? row,
    TResult? Function(List<McpComponent> children)? column,
  }) {
    return row?.call(children);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String title, String content)? card,
    TResult Function(String label, String actionId)? button,
    TResult Function(String content)? markdown,
    TResult Function(List<McpComponent> children)? row,
    TResult Function(List<McpComponent> children)? column,
    required TResult orElse(),
  }) {
    if (row != null) {
      return row(children);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(McpComponent_Card value) card,
    required TResult Function(McpComponent_Button value) button,
    required TResult Function(McpComponent_Markdown value) markdown,
    required TResult Function(McpComponent_Row value) row,
    required TResult Function(McpComponent_Column value) column,
  }) {
    return row(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(McpComponent_Card value)? card,
    TResult? Function(McpComponent_Button value)? button,
    TResult? Function(McpComponent_Markdown value)? markdown,
    TResult? Function(McpComponent_Row value)? row,
    TResult? Function(McpComponent_Column value)? column,
  }) {
    return row?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(McpComponent_Card value)? card,
    TResult Function(McpComponent_Button value)? button,
    TResult Function(McpComponent_Markdown value)? markdown,
    TResult Function(McpComponent_Row value)? row,
    TResult Function(McpComponent_Column value)? column,
    required TResult orElse(),
  }) {
    if (row != null) {
      return row(this);
    }
    return orElse();
  }
}

abstract class McpComponent_Row extends McpComponent {
  const factory McpComponent_Row({required final List<McpComponent> children}) =
      _$McpComponent_RowImpl;
  const McpComponent_Row._() : super._();

  List<McpComponent> get children;

  /// Create a copy of McpComponent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$McpComponent_RowImplCopyWith<_$McpComponent_RowImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$McpComponent_ColumnImplCopyWith<$Res> {
  factory _$$McpComponent_ColumnImplCopyWith(
    _$McpComponent_ColumnImpl value,
    $Res Function(_$McpComponent_ColumnImpl) then,
  ) = __$$McpComponent_ColumnImplCopyWithImpl<$Res>;
  @useResult
  $Res call({List<McpComponent> children});
}

/// @nodoc
class __$$McpComponent_ColumnImplCopyWithImpl<$Res>
    extends _$McpComponentCopyWithImpl<$Res, _$McpComponent_ColumnImpl>
    implements _$$McpComponent_ColumnImplCopyWith<$Res> {
  __$$McpComponent_ColumnImplCopyWithImpl(
    _$McpComponent_ColumnImpl _value,
    $Res Function(_$McpComponent_ColumnImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of McpComponent
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? children = null}) {
    return _then(
      _$McpComponent_ColumnImpl(
        children: null == children
            ? _value._children
            : children // ignore: cast_nullable_to_non_nullable
                  as List<McpComponent>,
      ),
    );
  }
}

/// @nodoc

class _$McpComponent_ColumnImpl extends McpComponent_Column {
  const _$McpComponent_ColumnImpl({required final List<McpComponent> children})
    : _children = children,
      super._();

  final List<McpComponent> _children;
  @override
  List<McpComponent> get children {
    if (_children is EqualUnmodifiableListView) return _children;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_children);
  }

  @override
  String toString() {
    return 'McpComponent.column(children: $children)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$McpComponent_ColumnImpl &&
            const DeepCollectionEquality().equals(other._children, _children));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(_children));

  /// Create a copy of McpComponent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$McpComponent_ColumnImplCopyWith<_$McpComponent_ColumnImpl> get copyWith =>
      __$$McpComponent_ColumnImplCopyWithImpl<_$McpComponent_ColumnImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String title, String content) card,
    required TResult Function(String label, String actionId) button,
    required TResult Function(String content) markdown,
    required TResult Function(List<McpComponent> children) row,
    required TResult Function(List<McpComponent> children) column,
  }) {
    return column(children);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String title, String content)? card,
    TResult? Function(String label, String actionId)? button,
    TResult? Function(String content)? markdown,
    TResult? Function(List<McpComponent> children)? row,
    TResult? Function(List<McpComponent> children)? column,
  }) {
    return column?.call(children);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String title, String content)? card,
    TResult Function(String label, String actionId)? button,
    TResult Function(String content)? markdown,
    TResult Function(List<McpComponent> children)? row,
    TResult Function(List<McpComponent> children)? column,
    required TResult orElse(),
  }) {
    if (column != null) {
      return column(children);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(McpComponent_Card value) card,
    required TResult Function(McpComponent_Button value) button,
    required TResult Function(McpComponent_Markdown value) markdown,
    required TResult Function(McpComponent_Row value) row,
    required TResult Function(McpComponent_Column value) column,
  }) {
    return column(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(McpComponent_Card value)? card,
    TResult? Function(McpComponent_Button value)? button,
    TResult? Function(McpComponent_Markdown value)? markdown,
    TResult? Function(McpComponent_Row value)? row,
    TResult? Function(McpComponent_Column value)? column,
  }) {
    return column?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(McpComponent_Card value)? card,
    TResult Function(McpComponent_Button value)? button,
    TResult Function(McpComponent_Markdown value)? markdown,
    TResult Function(McpComponent_Row value)? row,
    TResult Function(McpComponent_Column value)? column,
    required TResult orElse(),
  }) {
    if (column != null) {
      return column(this);
    }
    return orElse();
  }
}

abstract class McpComponent_Column extends McpComponent {
  const factory McpComponent_Column({
    required final List<McpComponent> children,
  }) = _$McpComponent_ColumnImpl;
  const McpComponent_Column._() : super._();

  List<McpComponent> get children;

  /// Create a copy of McpComponent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$McpComponent_ColumnImplCopyWith<_$McpComponent_ColumnImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
