<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Create project such that calculate gets triggered</description>
   <name>Create project such that calculate gets triggered</name>
   <tag></tag>
   <elementGuidId>5737ce29-2dbc-4a9f-897d-57f3cb40405d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\&quot;autoSelectTemplateFlag\&quot;: false,\n\&quot;category\&quot;: \&quot;Project\&quot;,\n\&quot;organizationId\&quot;: \&quot;${OrgId}\&quot;,\n\&quot;shortName\&quot;: \&quot;Project with matching Model\&quot;,\n\&quot;taskTemplateId\&quot;: null,\n\&quot;tzOffsetPlanned\&quot;: 330,\n\&quot;projectFieldMap\&quot;: {\n\&quot;25034\&quot;: \&quot;Test\&quot;,\n\&quot;25036\&quot;: \&quot;1\&quot;,\n\&quot;25038\&quot;: \&quot;a\&quot;,\n\&quot;25040\&quot;: \&quot;3218\&quot;,\n\&quot;25041\&quot;: \&quot;12/31/2018\&quot;,\n\&quot;25042\&quot;: \&quot;Yes\&quot;,\n\&quot;25056\&quot;: \&quot;Project\&quot;,\n\&quot;26376\&quot;: \&quot;1\&quot;\n}\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${Authorization}</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url}/rest/projects?</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>findTestData('Create Entity').getValue(1, 1)</defaultValue>
      <description></description>
      <id>2f6a49ef-dd71-490c-b082-597b0dc14d48</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>findTestData('Create Entity').getValue(2, 1)</defaultValue>
      <description></description>
      <id>6ce76f80-9267-494d-81c0-fd8ddd65ef65</id>
      <masked>false</masked>
      <name>OrgId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Authorization</defaultValue>
      <description></description>
      <id>3367005d-8e22-4744-a87b-5870eab48610</id>
      <masked>false</masked>
      <name>Authorization</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
