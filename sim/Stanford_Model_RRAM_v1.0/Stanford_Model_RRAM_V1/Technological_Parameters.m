function varargout = Technological_Parameters(varargin)
% TECHNOLOGICAL_PARAMETERS MATLAB code for Technological_Parameters.fig
%      TECHNOLOGICAL_PARAMETERS, by itself, creates a new TECHNOLOGICAL_PARAMETERS or raises the existing
%      singleton*.
%
%      H = TECHNOLOGICAL_PARAMETERS returns the handle to a new TECHNOLOGICAL_PARAMETERS or the handle to
%      the existing singleton*.
%
%      TECHNOLOGICAL_PARAMETERS('CALLBACK',hObject,eventData,handles,...) calls the local
%      function named CALLBACK in TECHNOLOGICAL_PARAMETERS.M with the given input arguments.
%
%      TECHNOLOGICAL_PARAMETERS('Property','Value',...) creates a new TECHNOLOGICAL_PARAMETERS or raises the
%      existing singleton*.  Starting from the left, property value pairs are
%      applied to the GUI before Technological_Parameters_OpeningFcn gets called.  An
%      unrecognized property name or invalid value makes property application
%      stop.  All inputs are passed to Technological_Parameters_OpeningFcn via varargin.
%
%      *See GUI Options on GUIDE's Tools menu.  Choose "GUI allows only one
%      instance to run (singleton)".
%
% See also: GUIDE, GUIDATA, GUIHANDLES

% Edit the above text to modify the response to help Technological_Parameters

% Last Modified by GUIDE v2.5 24-Jul-2018 09:19:56

% Begin initialization code - DO NOT EDIT
gui_Singleton = 1;
gui_State = struct('gui_Name',       mfilename, ...
                   'gui_Singleton',  gui_Singleton, ...
                   'gui_OpeningFcn', @Technological_Parameters_OpeningFcn, ...
                   'gui_OutputFcn',  @Technological_Parameters_OutputFcn, ...
                   'gui_LayoutFcn',  [] , ...
                   'gui_Callback',   []);
if nargin && ischar(varargin{1})
    gui_State.gui_Callback = str2func(varargin{1});
end

if nargout
    [varargout{1:nargout}] = gui_mainfcn(gui_State, varargin{:});
else
    gui_mainfcn(gui_State, varargin{:});
end
% End initialization code - DO NOT EDIT


% --- Executes just before Technological_Parameters is made visible.
function Technological_Parameters_OpeningFcn(hObject, eventdata, handles, varargin)
% This function has no output args, see OutputFcn.
% hObject    handle to figure
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)
% varargin   command line arguments to Technological_Parameters (see VARARGIN)

% Choose default command line output for Technological_Parameters
handles.output = hObject;

% Update handles structure
guidata(hObject, handles);

% UIWAIT makes Technological_Parameters wait for user response (see UIRESUME)
% uiwait(handles.figure1);


global Parameters

model_value = Parameters.Technological.Model_Value; % Obtaining manually introduced value
set(handles.edit_model,'String',num2str(model_value)); % Transforming the value form string to double

ini_t = Parameters.Technological.Ini_t; % Obtaining manually introduced value
set(handles.edit_ini_t,'String',num2str(ini_t)); % Transforming the value form string to double

tox = Parameters.Technological.Tox; % Obtaining manually introduced value
set(handles.edit_tox,'String',num2str(tox)); % Transforming the value form string to double

ini_gap = Parameters.Technological.Gap.Ini; % Obtaining manually introduced value
set(handles.edit_ini_gap,'String',num2str(ini_gap)); % Transforming the value form string to double

min_gap = Parameters.Technological.Gap.Min; % Obtaining manually introduced value
set(handles.edit_min_gap,'String',num2str(min_gap)); % Transforming the value form string to double

max_gap = Parameters.Technological.Gap.Max; % Obtaining manually introduced value
set(handles.edit_max_gap,'String',num2str(max_gap)); % Transforming the value form string to double

t_crit = Parameters.Technological.T_crit; % Obtaining manually introduced value
set(handles.edit_t_crit,'String',num2str(t_crit)); % Transforming the value form string to double

rth = Parameters.Technological.Rth; % Obtaining manually introduced value
set(handles.edit_rth,'String',num2str(rth)); % Transforming the value form string to double

min_field = Parameters.Technological.Min_Field; % Obtaining manually introduced value
set(handles.edit_min_field,'String',num2str(min_field)); % Transforming the value form string to double

Vel0 = Parameters.Technological.Vel0; % Obtaining manually introduced value
set(handles.edit_vel0,'String',num2str(Vel0)); % Transforming the value form string to double

gap0 = Parameters.Technological.Gap0; % Obtaining manually introduced value
set(handles.edit_gap0,'String',num2str(gap0)); % Transforming the value form string to double

t_smth = Parameters.Technological.T_smth; % Obtaining manually introduced value
set(handles.edit_t_smth,'String',num2str(t_smth)); % Transforming the value form string to double




% --- Outputs from this function are returned to the command line.
function varargout = Technological_Parameters_OutputFcn(hObject, eventdata, handles) 
% varargout  cell array for returning output args (see VARARGOUT);
% hObject    handle to figure
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)

% Get default command line output from handles structure
varargout{1} = handles.output;



function edit_model_Callback(hObject, eventdata, handles)
% hObject    handle to edit_model (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)

% Hints: get(hObject,'String') returns contents of edit_model as text
%        str2double(get(hObject,'String')) returns contents of edit_model as a double

global Parameters

model_value = get(handles.edit_model,'String'); % Obtaining manually introduced value
model_value = str2double(model_value); % Transforming the value form string to double

Parameters.Technological.Model_Value = model_value; % Saving the value


% --- Executes during object creation, after setting all properties.
function edit_model_CreateFcn(hObject, eventdata, handles)
% hObject    handle to edit_model (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    empty - handles not created until after all CreateFcns called

% Hint: edit controls usually have a white background on Windows.
%       See ISPC and COMPUTER.
if ispc && isequal(get(hObject,'BackgroundColor'), get(0,'defaultUicontrolBackgroundColor'))
    set(hObject,'BackgroundColor','white');
end



function edit_ini_t_Callback(hObject, eventdata, handles)
% hObject    handle to edit_ini_t (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)

% Hints: get(hObject,'String') returns contents of edit_ini_t as text
%        str2double(get(hObject,'String')) returns contents of edit_ini_t as a double

global Parameters

ini_t = get(handles.edit_ini_t,'String'); % Obtaining manually introduced value
ini_t = str2double(ini_t); % Transforming the value form string to double

Parameters.Technological.Ini_t = ini_t; % Saving the value


% --- Executes during object creation, after setting all properties.
function edit_ini_t_CreateFcn(hObject, eventdata, handles)
% hObject    handle to edit_ini_t (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    empty - handles not created until after all CreateFcns called

% Hint: edit controls usually have a white background on Windows.
%       See ISPC and COMPUTER.
if ispc && isequal(get(hObject,'BackgroundColor'), get(0,'defaultUicontrolBackgroundColor'))
    set(hObject,'BackgroundColor','white');
end



function edit_tox_Callback(hObject, eventdata, handles)
% hObject    handle to edit_tox (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)

% Hints: get(hObject,'String') returns contents of edit_tox as text
%        str2double(get(hObject,'String')) returns contents of edit_tox as a double

global Parameters

tox = get(handles.edit_tox,'String'); % Obtaining manually introduced value
tox = str2double(tox); % Transforming the value form string to double

Parameters.Technological.Tox = tox; % Saving the value


% --- Executes during object creation, after setting all properties.
function edit_tox_CreateFcn(hObject, eventdata, handles)
% hObject    handle to edit_tox (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    empty - handles not created until after all CreateFcns called

% Hint: edit controls usually have a white background on Windows.
%       See ISPC and COMPUTER.
if ispc && isequal(get(hObject,'BackgroundColor'), get(0,'defaultUicontrolBackgroundColor'))
    set(hObject,'BackgroundColor','white');
end



function edit_ini_gap_Callback(hObject, eventdata, handles)
% hObject    handle to edit_ini_gap (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)

% Hints: get(hObject,'String') returns contents of edit_ini_gap as text
%        str2double(get(hObject,'String')) returns contents of edit_ini_gap as a double

global Parameters

ini_gap = get(handles.edit_ini_gap,'String'); % Obtaining manually introduced value
ini_gap = str2double(ini_gap); % Transforming the value form string to double

Parameters.Technological.Gap.Ini = ini_gap; % Saving the value


% --- Executes during object creation, after setting all properties.
function edit_ini_gap_CreateFcn(hObject, eventdata, handles)
% hObject    handle to edit_ini_gap (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    empty - handles not created until after all CreateFcns called

% Hint: edit controls usually have a white background on Windows.
%       See ISPC and COMPUTER.
if ispc && isequal(get(hObject,'BackgroundColor'), get(0,'defaultUicontrolBackgroundColor'))
    set(hObject,'BackgroundColor','white');
end



function edit_min_gap_Callback(hObject, eventdata, handles)
% hObject    handle to edit_min_gap (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)

% Hints: get(hObject,'String') returns contents of edit_min_gap as text
%        str2double(get(hObject,'String')) returns contents of edit_min_gap as a double

global Parameters

min_gap = get(handles.edit_min_gap,'String'); % Obtaining manually introduced value
min_gap = str2double(min_gap); % Transforming the value form string to double

Parameters.Technological.Gap.Min = min_gap; % Saving the value


% --- Executes during object creation, after setting all properties.
function edit_min_gap_CreateFcn(hObject, eventdata, handles)
% hObject    handle to edit_min_gap (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    empty - handles not created until after all CreateFcns called

% Hint: edit controls usually have a white background on Windows.
%       See ISPC and COMPUTER.
if ispc && isequal(get(hObject,'BackgroundColor'), get(0,'defaultUicontrolBackgroundColor'))
    set(hObject,'BackgroundColor','white');
end



function edit_max_gap_Callback(hObject, eventdata, handles)
% hObject    handle to edit_max_gap (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)

% Hints: get(hObject,'String') returns contents of edit_max_gap as text
%        str2double(get(hObject,'String')) returns contents of edit_max_gap as a double

global Parameters

max_gap = get(handles.edit_max_gap,'String'); % Obtaining manually introduced value
max_gap = str2double(max_gap); % Transforming the value form string to double

Parameters.Technological.Gap.Max = max_gap; % Saving the value


% --- Executes during object creation, after setting all properties.
function edit_max_gap_CreateFcn(hObject, eventdata, handles)
% hObject    handle to edit_max_gap (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    empty - handles not created until after all CreateFcns called

% Hint: edit controls usually have a white background on Windows.
%       See ISPC and COMPUTER.
if ispc && isequal(get(hObject,'BackgroundColor'), get(0,'defaultUicontrolBackgroundColor'))
    set(hObject,'BackgroundColor','white');
end



function edit_t_crit_Callback(hObject, eventdata, handles)
% hObject    handle to edit_t_crit (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)

% Hints: get(hObject,'String') returns contents of edit_t_crit as text
%        str2double(get(hObject,'String')) returns contents of edit_t_crit as a double

global Parameters

t_crit = get(handles.edit_t_crit,'String'); % Obtaining manually introduced value
t_crit = str2double(t_crit); % Transforming the value form string to double

Parameters.Technological.T_crit = t_crit; % Saving the value


% --- Executes during object creation, after setting all properties.
function edit_t_crit_CreateFcn(hObject, eventdata, handles)
% hObject    handle to edit_t_crit (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    empty - handles not created until after all CreateFcns called

% Hint: edit controls usually have a white background on Windows.
%       See ISPC and COMPUTER.
if ispc && isequal(get(hObject,'BackgroundColor'), get(0,'defaultUicontrolBackgroundColor'))
    set(hObject,'BackgroundColor','white');
end



function edit_rth_Callback(hObject, eventdata, handles)
% hObject    handle to edit_rth (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)

% Hints: get(hObject,'String') returns contents of edit_rth as text
%        str2double(get(hObject,'String')) returns contents of edit_rth as a double

global Parameters

rth = get(handles.edit_rth,'String'); % Obtaining manually introduced value
rth = str2double(rth); % Transforming the value form string to double

Parameters.Technological.Rth = rth; % Saving the value


% --- Executes during object creation, after setting all properties.
function edit_rth_CreateFcn(hObject, eventdata, handles)
% hObject    handle to edit_rth (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    empty - handles not created until after all CreateFcns called

% Hint: edit controls usually have a white background on Windows.
%       See ISPC and COMPUTER.
if ispc && isequal(get(hObject,'BackgroundColor'), get(0,'defaultUicontrolBackgroundColor'))
    set(hObject,'BackgroundColor','white');
end



function edit_min_field_Callback(hObject, eventdata, handles)
% hObject    handle to edit_min_field (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)

% Hints: get(hObject,'String') returns contents of edit_min_field as text
%        str2double(get(hObject,'String')) returns contents of edit_min_field as a double

global Parameters

min_field = get(handles.edit_min_field,'String'); % Obtaining manually introduced value
min_field = str2double(min_field); % Transforming the value form string to double

Parameters.Technological.Min_Field = min_field; % Saving the value


% --- Executes during object creation, after setting all properties.
function edit_min_field_CreateFcn(hObject, eventdata, handles)
% hObject    handle to edit_min_field (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    empty - handles not created until after all CreateFcns called

% Hint: edit controls usually have a white background on Windows.
%       See ISPC and COMPUTER.
if ispc && isequal(get(hObject,'BackgroundColor'), get(0,'defaultUicontrolBackgroundColor'))
    set(hObject,'BackgroundColor','white');
end



function edit_vel0_Callback(hObject, eventdata, handles)
% hObject    handle to edit_vel0 (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)

% Hints: get(hObject,'String') returns contents of edit_vel0 as text
%        str2double(get(hObject,'String')) returns contents of edit_vel0 as a double

global Parameters

vel0 = get(handles.edit_vel0,'String'); % Obtaining manually introduced value
vel0 = str2double(vel0); % Transforming the value form string to double

Parameters.Technological.Vel0 = vel0; % Saving the value


% --- Executes during object creation, after setting all properties.
function edit_vel0_CreateFcn(hObject, eventdata, handles)
% hObject    handle to edit_vel0 (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    empty - handles not created until after all CreateFcns called

% Hint: edit controls usually have a white background on Windows.
%       See ISPC and COMPUTER.
if ispc && isequal(get(hObject,'BackgroundColor'), get(0,'defaultUicontrolBackgroundColor'))
    set(hObject,'BackgroundColor','white');
end



function edit_gap0_Callback(hObject, eventdata, handles)
% hObject    handle to edit_gap0 (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)

% Hints: get(hObject,'String') returns contents of edit_gap0 as text
%        str2double(get(hObject,'String')) returns contents of edit_gap0 as a double

global Parameters

gap0 = get(handles.edit_gap0,'String'); % Obtaining manually introduced value
gap0 = str2double(gap0); % Transforming the value form string to double

Parameters.Technological.Gap0 = gap0; % Saving the value


% --- Executes during object creation, after setting all properties.
function edit_gap0_CreateFcn(hObject, eventdata, handles)
% hObject    handle to edit_gap0 (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    empty - handles not created until after all CreateFcns called

% Hint: edit controls usually have a white background on Windows.
%       See ISPC and COMPUTER.
if ispc && isequal(get(hObject,'BackgroundColor'), get(0,'defaultUicontrolBackgroundColor'))
    set(hObject,'BackgroundColor','white');
end



function edit_t_smth_Callback(hObject, eventdata, handles)
% hObject    handle to edit_t_smth (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)

% Hints: get(hObject,'String') returns contents of edit_t_smth as text
%        str2double(get(hObject,'String')) returns contents of edit_t_smth as a double

global Parameters

t_smth = get(handles.edit_t_smth,'String'); % Obtaining manually introduced value
t_smth = str2double(t_smth); % Transforming the value form string to double

Parameters.Technological.T_smth = t_smth; % Saving the value


% --- Executes during object creation, after setting all properties.
function edit_t_smth_CreateFcn(hObject, eventdata, handles)
% hObject    handle to edit_t_smth (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    empty - handles not created until after all CreateFcns called

% Hint: edit controls usually have a white background on Windows.
%       See ISPC and COMPUTER.
if ispc && isequal(get(hObject,'BackgroundColor'), get(0,'defaultUicontrolBackgroundColor'))
    set(hObject,'BackgroundColor','white');
end


% --- Executes when user attempts to close figure1.
function figure1_CloseRequestFcn(hObject, eventdata, handles)
% hObject    handle to figure1 (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)

% Hint: delete(hObject) closes the figure

global Parameters

save('Parameters.mat','Parameters');

delete(hObject);
