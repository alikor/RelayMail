{{- define "relaymail-email-ses.name" -}}
{{- default .Chart.Name .Values.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{- define "relaymail-email-ses.fullname" -}}
{{- if .Values.fullnameOverride -}}
{{- .Values.fullnameOverride | trunc 63 | trimSuffix "-" -}}
{{- else -}}
{{- $name := default .Chart.Name .Values.nameOverride -}}
{{- printf "%s-%s" .Release.Name $name | trunc 63 | trimSuffix "-" -}}
{{- end -}}
{{- end -}}

{{/* Selector labels — must stay stable across upgrades. */}}
{{- define "relaymail-email-ses.selectorLabels" -}}
app.kubernetes.io/name: {{ include "relaymail-email-ses.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end -}}

{{/* Full label set applied to every resource. */}}
{{- define "relaymail-email-ses.labels" -}}
{{ include "relaymail-email-ses.selectorLabels" . }}
app.kubernetes.io/version: {{ .Chart.AppVersion | quote }}
app.kubernetes.io/part-of: relaymail
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end -}}
